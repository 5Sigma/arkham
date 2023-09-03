use std::{cell::RefCell, rc::Rc};

use crate::{
    container::Container,
    prelude::{Callable, Pos, Runes, Size, ViewContext},
    view::View,
};

#[derive(Debug, Clone, Copy)]
pub enum StackDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct Stack {
    pub(crate) direction: StackDirection,
    pub(crate) container: Rc<RefCell<Container>>,
    pub(crate) view: View,
    pub(crate) position: Pos,
}

impl Stack {
    pub fn component<F, Args, S>(&mut self, size: S, f: F)
    where
        F: crate::prelude::Callable<Args>,
        Args: crate::prelude::FromContainer,
        S: Into<Size>,
    {
        let size = size.into();
        let mut context = ViewContext::new(self.container.clone(), size);
        f.call(&mut context, Args::from_container(&self.container.borrow()));
        self.view.apply(self.position, &context.view);
        self.position += match self.direction {
            StackDirection::Vertical => Pos::new(0, size.height),
            StackDirection::Horizontal => Pos::new(size.width, 0),
        };
    }

    pub fn insert<R: Into<Runes>>(&mut self, value: R) {
        let runes: Runes = value.into();
        let l = runes.len();
        self.view.insert(self.position, runes);
        self.position += match self.direction {
            StackDirection::Vertical => Pos::new(0, 1),
            StackDirection::Horizontal => Pos::new(l, 0),
        };
    }
}

impl Callable<()> for Stack {
    fn call(&self, ctx: &mut ViewContext, _args: ()) {
        ctx.apply((0, 0), &self.view);
    }
}
