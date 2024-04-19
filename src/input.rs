use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyModifiers};

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

    /// Set the keyboard state to indicate a specific keycode is pressed
    pub(crate) fn set_key(&self, k: KeyCode) {
        *self.key.borrow_mut() = Some(k);
    }

    /// Set the keyboard state to indicate specific modifier keys are pressed
    pub(crate) fn set_modifiers(&self, modifiers: KeyModifiers) {
        *self.modifiers.borrow_mut() = modifiers;
    }

    /// Resets the keyboard state. This can be used after accepting
    /// a keypress within a component to prevent further components from
    /// registering the keypress event
    pub fn reset(&self) {
        *self.key.borrow_mut() = None;
    }

    /// Retruns the keycode that is current pressed, or None if there are
    /// no currently pressed keys
    pub fn code(&self) -> Option<KeyCode> {
        *self.key.borrow()
    }

    /// Returns the char value of the pressed key. Returns None if no key
    /// is currently pressed, or if the key does not have a char value.
    pub fn char(&self) -> Option<char> {
        if let Some(KeyCode::Char(c)) = *self.key.borrow() {
            Some(c)
        } else {
            None
        }
    }

    /// Returns true if the shift key is current pressed
    pub fn shift(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::SHIFT)
    }

    /// Returns true if the control key is current pressed
    pub fn control(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::CONTROL)
    }

    /// Returns true if the alt key is current pressed
    pub fn alt(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::ALT)
    }

    /// Returns true if the super key is current pressed
    pub fn super_key(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::SUPER)
    }

    /// Returns true if the hyper key is current pressed
    pub fn hyper(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::HYPER)
    }

    /// Returns true if the meta key is current pressed
    pub fn meta(&self) -> bool {
        self.modifiers.borrow().contains(KeyModifiers::META)
    }
}
