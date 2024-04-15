use std::{cell::RefCell, rc::Rc};

use crate::{
    container::Container,
    prelude::{Callable, Pos, Runes, Size, ViewContext},
    view::View,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum StackAlignment {
    #[default]
    Left,
    Right,
    Center,
    Top,
    Bottom,
}

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
    pub(crate) alignment: StackAlignment,
}

impl Stack {
    pub fn alignment(&mut self, alignment: StackAlignment) {
        self.alignment = alignment;
    }

    pub fn component<F, Args, S>(&mut self, size: S, f: F)
    where
        F: crate::prelude::Callable<Args>,
        Args: crate::prelude::FromContainer,
        S: Into<Size>,
    {
        let size = size.into();

        let pos = match self.direction {
            StackDirection::Vertical => {
                if size.width != self.view.size().width {
                    match self.alignment {
                        StackAlignment::Left | StackAlignment::Top | StackAlignment::Bottom => {
                            self.position
                        }
                        StackAlignment::Right => Pos::new(
                            self.position.x + self.view.size().width - size.width,
                            self.position.y,
                        ),
                        StackAlignment::Center => {
                            let view_width = self.view.size().width as f32;
                            let diff = view_width - size.width as f32;
                            Pos::new(
                                self.position.x + (diff / 2.0).floor() as usize,
                                self.position.y,
                            )
                        }
                    }
                } else {
                    self.position
                }
            }
            StackDirection::Horizontal => {
                if size.height != self.view.size().height {
                    match self.alignment {
                        StackAlignment::Top | StackAlignment::Left | StackAlignment::Right => {
                            self.position
                        }
                        StackAlignment::Bottom => Pos::new(
                            self.position.x,
                            self.position.y + self.view.size().height - size.height,
                        ),
                        StackAlignment::Center => {
                            let view_height = self.view.size().height as f32;
                            let diff = view_height - size.height as f32;
                            Pos::new(
                                self.position.x,
                                self.position.y + (diff / 2.0).floor() as usize,
                            )
                        }
                    }
                } else {
                    self.position
                }
            }
        };

        let mut context = ViewContext::new(self.container.clone(), size);
        f.call(&mut context, Args::from_container(&self.container.borrow()));
        self.view.apply(pos, &context.view);
        self.position += match self.direction {
            StackDirection::Vertical => Pos::new(0, size.height),
            StackDirection::Horizontal => Pos::new(size.width, 0),
        };
    }

    /// Insert a set a runes, such as a string, into the stack.
    pub fn insert<R: Into<Runes>>(&mut self, value: R) {
        let runes: Runes = value.into();
        let size = Size::new(runes.len(), 1);

        let pos = match self.direction {
            StackDirection::Vertical => {
                if size.width != self.view.size().width {
                    match self.alignment {
                        StackAlignment::Left | StackAlignment::Top | StackAlignment::Bottom => {
                            self.position
                        }

                        StackAlignment::Right => Pos::new(
                            self.position.x + self.view.size().width - size.width,
                            self.position.y,
                        ),
                        StackAlignment::Center => {
                            let view_width = self.view.size().width as f32;
                            let diff = view_width - size.width as f32;
                            Pos::new(
                                self.position.x + (diff / 2.0).floor() as usize,
                                self.position.y,
                            )
                        }
                    }
                } else {
                    self.position
                }
            }
            StackDirection::Horizontal => {
                if size.height != self.view.size().height {
                    match self.alignment {
                        StackAlignment::Top | StackAlignment::Left | StackAlignment::Right => {
                            self.position
                        }
                        StackAlignment::Bottom => Pos::new(
                            self.position.x,
                            self.position.y + self.view.size().height - size.height,
                        ),
                        StackAlignment::Center => {
                            let view_height = self.view.size().height as f32;
                            let diff = view_height - size.height as f32;
                            Pos::new(
                                self.position.x,
                                self.position.y + (diff / 2.0).floor() as usize,
                            )
                        }
                    }
                } else {
                    self.position
                }
            }
        };

        self.view.insert(pos, runes);
        self.position += match self.direction {
            StackDirection::Vertical => Pos::new(0, 1),
            StackDirection::Horizontal => Pos::new(size.width, 0),
        };
    }
}

impl Callable<()> for Stack {
    fn call(&self, ctx: &mut ViewContext, _args: ()) {
        ctx.apply((0, 0), &self.view);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{StackAlignment, ViewContext};

    #[test]
    fn test_vertical_insert() {
        let ctx = crate::context::tests::context_fixture();
        let mut stack = ctx.vertical_stack((10, 2).into());
        stack.insert("one");
        stack.insert("two");
        assert_eq!(
            stack.view.render_text(),
            "one\0\0\0\0\0\0\0\ntwo\0\0\0\0\0\0\0\n".to_string()
        );
    }

    #[test]
    fn test_horizontal_insert() {
        let ctx = crate::context::tests::context_fixture();
        let mut stack = ctx.horizontal_stack((10, 1).into());
        stack.insert("one");
        stack.insert("two");
        assert_eq!(stack.view.render_text(), "onetwo\0\0\0\0\n".to_string());
    }

    #[test]
    fn test_component() {
        let ctx = crate::context::tests::context_fixture();
        let mut stack = ctx.horizontal_stack((10, 2).into());
        stack.component((10, 2), |ctx: &mut ViewContext| {
            ctx.insert((3, 1), "one");
        });
        assert_eq!(
            stack.view.render_text(),
            "\0\0\0\0\0\0\0\0\0\0\n\0\0\0one\0\0\0\0\n".to_string()
        );
    }

    #[test]
    fn test_align_left() {
        let ctx = crate::context::tests::context_fixture();
        let mut stack = ctx.vertical_stack((10, 2).into());
        stack.component((5, 1), |ctx: &mut ViewContext| {
            ctx.insert((0, 0), "one");
        });
        stack.insert("two");
        assert_eq!(
            stack.view.render_text(),
            "one\0\0\0\0\0\0\0\ntwo\0\0\0\0\0\0\0\n".to_string()
        );
    }

    #[test]
    fn test_align_right() {
        let ctx = crate::context::tests::context_fixture();
        let mut stack = ctx.vertical_stack((10, 3).into());
        stack.alignment = StackAlignment::Right;
        stack.insert("one");
        stack.component((5, 1), |ctx: &mut ViewContext| {
            ctx.insert((0, 0), "one");
        });
        stack.component((10, 1), |ctx: &mut ViewContext| {
            ctx.insert((0, 0), "two");
        });

        let res = "\0\0\0\0\0\0\0one\n\0\0\0\0\0one\0\0\ntwo\0\0\0\0\0\0\0\n".to_string();

        crate::tests::print_render_text(&stack.view.render_text());
        println!("---");
        crate::tests::print_render_text(&res);

        assert_eq!(stack.view.render_text(), res);
    }

    #[test]
    fn test_align_center_v() {
        let ctx = crate::context::tests::context_fixture();
        let mut stack = ctx.vertical_stack((10, 3).into());
        stack.alignment = StackAlignment::Center;
        stack.insert("one");
        stack.component((5, 1), |ctx: &mut ViewContext| {
            ctx.insert((0, 0), "one");
        });
        stack.component((10, 1), |ctx: &mut ViewContext| {
            ctx.insert((0, 0), "two");
        });

        let res = "\0\0\0one\0\0\0\0\n\0\0one\0\0\0\0\0\ntwo\0\0\0\0\0\0\0\n".to_string();

        crate::tests::print_render_text(&stack.view.render_text());
        println!("---");
        crate::tests::print_render_text(&res);

        assert_eq!(stack.view.render_text(), res);
    }

    #[test]
    fn test_align_top() {
        let ctx = crate::context::tests::context_fixture();
        let mut stack = ctx.horizontal_stack((9, 6).into());
        stack.component((3, 1), |ctx: &mut ViewContext| {
            ctx.insert((0, 0), "one");
        });
        stack.component((3, 3), |ctx: &mut ViewContext| {
            ctx.insert((0, 1), "two");
        });
        stack.insert("one");

        let res = "one\0\0\0one\n\0\0\0two\0\0\0\n\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\n"
            .to_string();

        crate::tests::print_render_text(&stack.view.render_text());
        println!("---");
        crate::tests::print_render_text(&res);

        assert_eq!(stack.view.render_text(), res.to_string());
    }

    #[test]
    fn test_align_bottom() {
        let ctx = crate::context::tests::context_fixture();
        let mut stack = ctx.horizontal_stack((9, 6).into());
        stack.alignment(StackAlignment::Bottom);
        stack.component((3, 1), |ctx: &mut ViewContext| {
            ctx.insert((0, 0), "one");
        });
        stack.component((3, 3), |ctx: &mut ViewContext| {
            ctx.insert((0, 1), "two");
        });
        stack.insert("one");

        let res = "\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\n\0\0\0two\0\0\0\none\0\0\0one\n"
            .to_string();

        crate::tests::print_render_text(&stack.view.render_text());
        println!("---");
        crate::tests::print_render_text(&res);

        assert_eq!(stack.view.render_text(), res);
    }

    #[test]
    fn test_align_center_h() {
        let ctx = crate::context::tests::context_fixture();
        let mut stack = ctx.horizontal_stack((9, 6).into());
        stack.alignment(StackAlignment::Center);
        stack.component((3, 1), |ctx: &mut ViewContext| {
            ctx.insert((0, 0), "one");
        });
        stack.component((3, 3), |ctx: &mut ViewContext| {
            ctx.insert((0, 1), "two");
        });
        stack.insert("one");

        let res = "\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\nonetwoone\n\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\n\0\0\0\0\0\0\0\0\0\n"
            .to_string();

        crate::tests::print_render_text(&stack.view.render_text());
        println!("---");
        crate::tests::print_render_text(&res);

        assert_eq!(stack.view.render_text(), res);
    }
}
