use std::{
    any::Any,
    cell::RefCell,
    io::Write,
    marker::PhantomData,
    rc::Rc,
    sync::mpsc::{channel, Receiver, Sender},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEventKind},
    execute, queue, terminal,
};

use crate::{
    container::{Callable, Container, ContainerRef, FromContainer, Res, State},
    context::ViewContext,
    plugins::Plugin,
    runes::Rune,
    view::View,
};

use super::input::Keyboard;

/// A renderer that can signal a render needs to take place.
pub struct Renderer {
    tx: Sender<()>,
}

impl Renderer {
    pub fn render(&self) {
        let _ = self.tx.send(());
    }
}

struct AppOptions {
    q_to_quit: bool,
}

impl Default for AppOptions {
    fn default() -> Self {
        Self { q_to_quit: true }
    }
}

/// The app is the core container for the application logic, resources,
/// state, and run loop.
///
/// Setting up a basic application:
///
/// ```no_run
/// use arkham::prelude::*;
///
/// fn main() {
///     App::new(root_view).run();
/// }
///
/// fn root_view(ctx: &mut ViewContext) {
///     ctx.insert((2,2), "Hello World");
/// }
/// ```
pub struct App<F, Args>
where
    F: Callable<Args>,
    Args: FromContainer,
{
    options: AppOptions,
    container: ContainerRef,
    main_view: View,
    current_view_state: Vec<Vec<Rune>>,
    render_signal: Receiver<()>,
    render_tx: Sender<()>,
    root: F,
    args: PhantomData<Args>,
    plugins: Rc<RefCell<Vec<Box<dyn crate::plugins::Plugin>>>>,
}

impl<F, Args> App<F, Args>
where
    F: Callable<Args>,
    Args: FromContainer,
{
    /// Constructs a new App objcet. This object uses a builder pattern and
    /// should be finalized with App::run(). which will start a blocking run
    /// loop and perform the initial screen setup and render.
    pub fn new(root: F) -> App<F, Args> {
        let container = Rc::new(RefCell::new(Container::default()));
        let size = terminal::size().unwrap();
        let main_view = View::new(size);
        let (render_tx, render_signal) = channel();

        App {
            container,
            root,
            main_view,
            current_view_state: vec![vec![Rune::default(); size.0 as usize]; size.1 as usize],
            render_tx,
            render_signal,
            options: AppOptions::default(),
            args: PhantomData,
            plugins: Rc::new(RefCell::new(vec![])),
        }
    }

    /// Disables the default handling of the 'q' key to quit the application
    ///
    /// NOTE: You will need to manually handle quitting via the ViewContext::exit function.
    pub fn disbale_q_to_quit(mut self) -> Self {
        self.options.q_to_quit = false;
        self
    }

    /// Returns a renderer that can signal the application to rerender. This
    /// renderer can be cloned and passed between threads.
    pub fn get_renderer(&self) -> Renderer {
        Renderer {
            tx: self.render_tx.clone(),
        }
    }

    pub fn insert_plugin(self, plugin: impl Plugin + 'static) -> Self {
        self.plugins.borrow_mut().push(Box::new(plugin));
        self
    }

    /// Insert a resource which can be injected into component functions.
    ///
    /// This resource can only be accessed immutably by reference.
    /// Interior mutability must be used for anything that requires an internal
    /// state.
    ///
    /// Example:
    /// ```no_run
    /// use arkham::prelude::*;
    /// struct MyResource {
    ///   value: i32
    /// }
    ///
    /// fn main() {
    ///     App::new(root).insert_resource(MyResource { value: 12 });
    /// }
    ///
    /// fn root(ctx: &mut ViewContext, thing: Res<MyResource>)  {
    ///     ctx.insert(0,format!("Value is {}", thing.value));
    /// }
    /// ````
    /// Alternatively, App::insert_state can be used to insert a state object,
    /// that can be borrowed mutable.
    pub fn insert_resource<T: Any>(self, v: T) -> Self {
        self.bind_resource(Res::new(v))
    }

    /// Bind an existing resource to the application
    ///
    /// Similar to `App::insert_resource` except it accepts an existing resource.
    pub fn bind_resource<T: Any>(self, v: Res<T>) -> Self {
        self.container.borrow_mut().bind(v);
        self
    }

    /// Insert a stateful object that can be injected into component functions
    /// unlike App::insert_resource, this value can be borrowed mutably and
    /// is meant to store application state.
    ///
    /// Example:
    /// ```no_run
    /// use arkham::prelude::*;
    /// struct MyState {
    ///   value: i32
    /// }
    ///
    /// fn main() {
    ///     App::new(root).insert_state(MyState { value: 12 });
    /// }
    ///
    /// fn root(ctx: &mut ViewContext, thing: State<MyState>)  {
    ///     thing.get_mut().value += 1;
    ///     ctx.insert(0,format!("Value is {}", thing.get().value));
    /// }
    /// ````
    pub fn insert_state<T: Any>(self, v: T) -> Self {
        self.bind_state(State::new(v))
    }

    /// Binds an existing state to the application.
    ///
    /// Similar to `App::insert_state` but will accept an existing state
    pub fn bind_state<T: Any>(self, v: State<T>) -> Self {
        self.container.borrow_mut().bind(v);
        self
    }

    /// Executes the main run loop. This should be called to start the
    /// application logic.
    ///
    /// This function will block while it reads events and performs render
    /// cycles.
    pub fn run(&mut self) -> anyhow::Result<()> {
        self.container.borrow_mut().bind(Res::new(Terminal));
        self.container.borrow_mut().bind(Res::new(Keyboard::new()));

        let _result = std::panic::catch_unwind(teardown);
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            teardown();
            default_hook(info);
        }));

        for plugin in self.plugins.borrow_mut().iter_mut() {
            plugin.build(self.container.clone());
        }

        let _ = ctrlc::set_handler(|| {
            let mut out = std::io::stdout();
            let _ = terminal::disable_raw_mode();
            let _ = execute!(out, terminal::LeaveAlternateScreen, cursor::Show);
            std::process::exit(0);
        });

        let mut out = std::io::stdout();
        execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;
        terminal::enable_raw_mode()?;
        self.render()?;

        loop {
            if crossterm::event::poll(Duration::from_millis(1000)).unwrap_or(false) {
                if let Ok(event) = crossterm::event::read() {
                    match event {
                        Event::FocusGained => self.render()?,
                        Event::FocusLost => {}
                        Event::Key(key_event) if key_event.code == KeyCode::Char('q') => {
                            if self.options.q_to_quit {
                                break;
                            }
                        }
                        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                            {
                                let container = self.container.borrow();
                                let kb = container.get::<Res<Keyboard>>().unwrap();
                                kb.set_key(key_event.code);
                                kb.set_modifiers(key_event.modifiers);
                            }
                            self.render()?;
                            self.render()?;
                        }
                        Event::Mouse(_) => todo!(),
                        Event::Paste(_) => todo!(),
                        Event::Resize(col, row) => {
                            self.main_view.0 =
                                vec![vec![Rune::default(); col as usize]; row as usize];
                            self.current_view_state =
                                vec![vec![Rune::default(); col as usize]; row as usize];
                            self.clear()?;
                            self.render()?
                        }
                        _ => {}
                    }
                }
            }
            if self.render_signal.try_recv().is_ok() {
                self.render()?;
                self.render()?;
            }
        }
        teardown();

        Ok(())
    }

    fn render(&mut self) -> anyhow::Result<()> {
        loop {
            let mut context = ViewContext::new(self.container.clone(), self.main_view.size());

            for plugin in self.plugins.borrow().iter() {
                plugin.before_render(&mut context, self.container.clone());
            }

            self.root
                .call(&mut context, Args::from_container(&self.container.borrow()));

            if context.should_exit {
                teardown();
                std::process::exit(0);
            }

            self.main_view.apply((0, 0), &context.view);

            for plugin in self.plugins.borrow().iter() {
                plugin.after_render(&mut context, self.container.clone());
                self.main_view.apply((0, 0), &context.view);
            }

            self.container
                .borrow()
                .get::<Res<Keyboard>>()
                .unwrap()
                .reset();

            if !context.rerender {
                break;
            }
        }

        let mut out = std::io::stdout();
        for (row, line) in self.main_view.iter().enumerate() {
            for (col, rune) in line.iter().enumerate() {
                if &self.current_view_state[row][col] != rune {
                    queue!(out, cursor::MoveTo(col as u16, row as u16))?;
                    rune.render(&mut out)?;
                    self.current_view_state[row][col] = *rune;
                }
            }
        }
        out.flush()?;
        Ok(())
    }

    fn clear(&self) -> anyhow::Result<()> {
        let mut out = std::io::stdout();
        execute!(
            out,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )?;
        out.flush()?;
        Ok(())
    }
}

/// Repairs the terminal state so it operates properly.
fn teardown() {
    let mut out = std::io::stdout();
    let _ = terminal::disable_raw_mode();
    let _ = execute!(out, terminal::LeaveAlternateScreen, cursor::Show);
}

pub struct Terminal;

impl Terminal {
    pub fn set_title(&self, name: &str) {
        let _ = execute!(std::io::stdout(), terminal::SetTitle(name));
    }
    pub fn size(&self) -> (u16, u16) {
        crossterm::terminal::size().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "sync")]
    #[test]
    fn test_state_send() {
        use crate::prelude::{App, State, ViewContext};

        #[derive(Default)]
        struct S {
            #[allow(dead_code)]
            i: i32,
        }

        let root_view = |_: &mut ViewContext| {};

        let state = State::new(S::default());
        is_send(state);
    }

    #[allow(dead_code)]
    fn is_send(_: impl Send) {}
}
