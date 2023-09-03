use std::{cell::RefCell, rc::Rc};

use crate::{
    container::{Callable, FromContainer},
    stack::Stack,
};

use super::{
    container::Container,
    geometry::{Pos, Rect, Size},
    runes::Rune,
    view::View,
};

/// ViewContext represents the display context for a given area.
/// it maintains the drawing state for the region internally and is used
/// to generate a final view that is eventually rendered.
pub struct ViewContext {
    pub view: View,
    pub container: Rc<RefCell<Container>>,
    pub(crate) rerender: bool,
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
    /// Constructs a new ViewConext for a given area.  A container reference
    /// must also be passedo, so that component functions called
    /// from the context are injectable.
    pub fn new(container: Rc<RefCell<Container>>, size: Size) -> Self {
        let view = View::new(size);

        Self {
            view,
            container,
            rerender: false,
        }
    }

    /// Notify the application to rerender the view. This is useful after a
    /// state change that might affect other views.  
    pub fn render(&mut self) {
        self.rerender = true;
    }

    pub fn vertical_stack(&self, size: Size) -> Stack {
        Stack {
            direction: crate::stack::StackDirection::Vertical,
            container: self.container.clone(),
            view: View::new(size),
            position: Pos::from(0),
        }
    }

    pub fn horizontal_stack(&self, size: Size) -> Stack {
        Stack {
            direction: crate::stack::StackDirection::Horizontal,
            container: self.container.clone(),
            view: View::new(size),
            position: Pos::from(0),
        }
    }

    /// Execute a component function. The passed function will receive a new
    /// ViewContext for its size and can be injected with arguments.
    /// The context given to the component function will then be applied to
    /// the parent ViewContext at a given position.
    pub fn component<F, Args, R>(&mut self, rect: R, f: F)
    where
        F: Callable<Args>,
        Args: FromContainer,
        R: Into<Rect>,
    {
        let rect = rect.into();
        let mut context = ViewContext::new(self.container.clone(), rect.size);
        let args = Args::from_container(&self.container.borrow());
        f.call(&mut context, args);
        self.view.apply(rect.pos, &context.view);
        self.rerender = context.rerender;
    }

    /// Set a specific rune to a specific position. This function can be used
    /// to set a signle character. To set multiple runes at a time see the
    /// View::insert function.
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
