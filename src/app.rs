use std::{any::Any, cell::RefCell, io::Write, marker::PhantomData, rc::Rc};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue, terminal,
};

use crate::{
    container::{Callable, Container, FromContainer, Res, State},
    context::ViewContext,
    view::View,
};

use super::input::Keyboard;

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

    pub fn change_root(&mut self, root: F) {
        self.root = root;
    }

    pub fn insert_resource<T: Any>(self, v: T) -> Self {
        self.container.borrow_mut().bind(Res::new(v));
        self
    }

    pub fn insert_state<T: Any>(self, v: T) -> Self {
        self.container.borrow_mut().bind(State::new(v));
        self
    }

    fn teardown(&self) {
        let mut out = std::io::stdout();
        let _ = terminal::disable_raw_mode();
        let _ = execute!(out, terminal::LeaveAlternateScreen, cursor::Show);
    }

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
