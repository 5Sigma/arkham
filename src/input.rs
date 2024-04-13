use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyModifiers, ModifierKeyCode};

/// Keyboard can be used as an injectable resource that provides information
/// about the current keyboard state. This is the primary mechanism by which
/// applications can respond to keyboard input from users.
#[derive(Debug)]
pub struct Keyboard {
    key: Rc<RefCell<Option<KeyCode>>>,
    modifiers: Rc<RefCell<KeyModifiers>>,
}
impl Default for Keyboard {
    fn default() -> Self {
        Self {
            key: Rc::new(RefCell::new(None)),
            modifiers: Rc::new(RefCell::new(KeyModifiers::empty())),
        }
    }
}

impl Keyboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_key(&self, k: KeyCode) {
        *self.key.borrow_mut() = Some(k);
    }

    pub fn set_modifiers(&self, modifiers: KeyModifiers) {
        *self.modifiers.borrow_mut() = modifiers;
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

    pub fn shift(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::SHIFT)
    }

    pub fn control(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::CONTROL)
    }

    pub fn alt(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::ALT)
    }

    pub fn super_key(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::SUPER)
    }

    pub fn hyper(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::HYPER)
    }

    pub fn meta(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::META)
    }
}
