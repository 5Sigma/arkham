mod app;
mod container;
mod context;
mod geometry;
mod input;
mod runes;
mod stack;
pub mod symbols;
mod theme;
mod view;

pub mod prelude {
    pub use super::{
        app::{App, Renderer, Terminal},
        container::{Callable, FromContainer, Res, State},
        context::ViewContext,
        geometry::{Pos, Rect, Size},
        input::Keyboard,
        runes::{Rune, Runes, ToRuneExt},
        stack::StackAlignment,
        theme::Theme,
    };
    pub use crossterm::event::KeyCode;
    pub use crossterm::style::Color;
}

#[cfg(test)]
pub mod tests {
    pub fn print_render_text(s: &String) {
        println!("{}", s.replace('\0', " "));
    }
}
