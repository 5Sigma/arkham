mod app;
mod container;
mod context;
mod geometry;
mod input;
mod runes;
pub mod symbols;
mod theme;
mod view;

pub mod prelude {
    pub use super::{
        app::{App, Terminal},
        container::{Callable, FromContainer, Res, State},
        context::ViewContext,
        geometry::{Pos, Rect, Size},
        input::Keyboard,
        runes::{Rune, Runes, ToRuneExt},
        theme::Theme,
    };
    pub use crossterm::event::KeyCode;
    pub use crossterm::style::Color;
}
