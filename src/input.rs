use std::{cell::RefCell, rc::Rc};

use crossterm::event::KeyCode;

/// Keyboard can be used as an injectable resource that provides information
/// about the current keyboard state. This is the primary mechanism by which
/// applications can respond to keyboard input from users.
#[derive(Debug, Default)]
pub struct Keyboard {
    key: Rc<RefCell<Option<KeyCode>>>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_key(&self, k: KeyCode) {
        *self.key.borrow_mut() = Some(k);
    }

    pub fn reset(&self) {
        *self.key.borrow_mut() = None;
    }

    pub fn code(&self) -> Option<KeyCode> {
        *self.key.borrow()
    }

    pub fn char(&self) -> Option<char> {
        if let Some(KeyCode::Char(c)) = *self.key.borrow() {
            Some(c)
        } else {
            None
        }
    }
}
