use std::{cell::RefCell, rc::Rc};

use crate::{
    container::{Callable, FromContainer},
    widget::Widget,
};

use super::{
    container::Container,
    geometry::{Pos, Rect, Size},
    runes::Rune,
    view::View,
};

pub struct ViewContext {
    pub view: View,
    pub container: Rc<RefCell<Container>>,
}

impl std::ops::DerefMut for ViewContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.view
    }
}

impl std::ops::Deref for ViewContext {
    type Target = View;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl ViewContext {
    pub fn new(container: Rc<RefCell<Container>>, size: Size) -> Self {
        let view = View::new(size);

        Self { view, container }
    }

    pub fn component<F, Args>(&mut self, rect: Rect, f: F)
    where
        F: Callable<Args>,
        Args: FromContainer,
    {
        let mut context = ViewContext::new(self.container.clone(), rect.size);
        self.container.borrow().call(&mut context, &f);
        self.view.apply(rect.pos, context.view);
    }

    pub fn widget(&mut self, rect: Rect, mut widget: impl Widget) {
        let mut context = ViewContext::new(self.container.clone(), rect.size);
        widget.ui(&mut context);
        self.view.apply(rect.pos, context.view);
    }

    pub fn set_rune<P>(&mut self, pos: P, rune: Rune)
    where
        P: Into<Pos>,
    {
        let Pos { x, y } = pos.into();
        if let Some(r) = self.view.get_mut(y).and_then(|row| row.get_mut(x)) {
            *r = rune;
        }
    }
}
