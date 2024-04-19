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
    container::{Callable, Container, FromContainer, Res, State},
    context::ViewContext,
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
    container: Rc<RefCell<Container>>,
    main_view: View,
    render_signal: Receiver<()>,
    render_tx: Sender<()>,
    root: F,
    args: PhantomData<Args>,
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
            render_tx,
            render_signal,
            args: PhantomData,
        }
    }

    /// Returns a renderer that can signal the application to rerender. This
    /// renderer can be cloned and passed between threads.
    pub fn get_renderer(&self) -> Renderer {
        Renderer {
            tx: self.render_tx.clone(),
        }
    }

    /// Insert a resource which can be injected into component functions.
    ///
    /// This resource can only be accessed immutably by reference.
    /// Interior mutability must be used for anything that requires an internal
    /// state.
    ///
    /// Example:
    /// ```
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
        self.container.borrow_mut().bind(Res::new(v));
        self
    }

    /// Insert a stateful object that can be injected into component functions
    /// unlike App::insert_resource, this value can be borrowed mutably and
    /// is meant to store application state.
    ///
    /// Example:
    /// ```
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
        self.container.borrow_mut().bind(State::new(v));
        self
    }

    /// Repairs the terminal state so it operates properly.  
    fn teardown(&self) {
        let mut out = std::io::stdout();
        let _ = terminal::disable_raw_mode();
        let _ = execute!(out, terminal::LeaveAlternateScreen, cursor::Show);
    }

    /// Executes the main run loop. This should be called to start the
    /// application logic.
    ///
    /// This function will block while it reads events and performs render
    /// cycles.
    pub fn run(&mut self) -> anyhow::Result<()> {
        self.container.borrow_mut().bind(Res::new(Terminal));
        self.container.borrow_mut().bind(Res::new(Keyboard::new()));

        let _ = ctrlc::set_handler(|| {
            let mut out = std::io::stdout();
            let _ = terminal::disable_raw_mode();
            let _ = execute!(out, terminal::LeaveAlternateScreen, cursor::Show);
            std::process::exit(0);
        });

        let mut out = std::io::stdout();
        execute!(out, terminal::EnterAlternateScreen, cursor::Hide)?;
        terminal::enable_raw_mode()?;

        loop {
            loop {
                let mut context =
                    ViewContext::new(self.container.clone(), terminal::size().unwrap().into());

                self.root
                    .call(&mut context, Args::from_container(&self.container.borrow()));
                self.main_view.apply((0, 0), &context.view);
                self.render()?;

                if !context.rerender {
                    break;
                }
            }

            self.container
                .borrow()
                .get::<Res<Keyboard>>()
                .unwrap()
                .reset();

            if crossterm::event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(event) = crossterm::event::read() {
                    match event {
                        Event::FocusGained => self.render()?,
                        Event::FocusLost => {}
                        Event::Key(key_event) if key_event.code == KeyCode::Char('q') => {
                            break;
                        }
                        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                            let container = self.container.borrow();
                            let kb = container.get::<Res<Keyboard>>().unwrap();
                            kb.set_key(key_event.code);
                            kb.set_modifiers(key_event.modifiers);
                        }
                        Event::Mouse(_) => todo!(),
                        Event::Paste(_) => todo!(),
                        Event::Resize(_, _) => self.render()?,
                        _ => {}
                    }
                }
            }
            if self.render_signal.try_recv().is_ok() {
                self.render()?;
            }
        }
        self.teardown();

        Ok(())
    }

    fn render(&mut self) -> anyhow::Result<()> {
        let mut out = std::io::stdout();
        for (row, line) in self.main_view.iter().enumerate() {
            for (col, rune) in line.iter().enumerate() {
                queue!(out, cursor::MoveTo(col as u16, row as u16))?;
                rune.render(&mut out)?;
            }
        }
        out.flush()?;
        Ok(())
    }
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
