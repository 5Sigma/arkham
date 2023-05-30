use std::{any::Any, cell::RefCell, io::Write, marker::PhantomData, rc::Rc};

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

/// The app is the core container for the application logic, resources,
/// state, and run loop.
pub struct App<F, Args>
where
    F: Callable<Args>,
    Args: FromContainer,
{
    container: Rc<RefCell<Container>>,
    main_view: View,
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
        App {
            container,
            root,
            main_view,
            args: PhantomData::default(),
        }
    }

    /// Used to change the root function that is used during each render cycle.
    pub fn change_root(&mut self, root: F) {
        self.root = root;
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
            let mut context =
                ViewContext::new(self.container.clone(), terminal::size().unwrap().into());

            self.container.borrow().call(&mut context, &self.root);
            self.main_view.apply((0, 0), context.view);
            self.render()?;

            self.container
                .borrow()
                .get::<Res<Keyboard>>()
                .unwrap()
                .reset();

            if let Ok(event) = crossterm::event::read() {
                match event {
                    Event::FocusGained => todo!(),
                    Event::FocusLost => todo!(),
                    Event::Key(key_event) if key_event.code == KeyCode::Char('q') => {
                        break;
                    }
                    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                        let container = self.container.borrow();
                        let kb = container.get::<Res<Keyboard>>().unwrap();
                        kb.set_key(key_event.code);
                    }
                    Event::Mouse(_) => todo!(),
                    Event::Paste(_) => todo!(),
                    Event::Resize(_, _) => todo!(),
                    _ => {}
                }
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
