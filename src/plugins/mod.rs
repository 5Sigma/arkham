use crate::prelude::ViewContext;

pub mod keybind;

pub trait Pluigin {
    fn build(ctx: ViewContext);

    #[allow(unused_variables)]
    fn before_render(ctx: ViewContext) {}

    #[allow(unused_variables)]
    fn after_render(ctx: ViewContext) {}
}
