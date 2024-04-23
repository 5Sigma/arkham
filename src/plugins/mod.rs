use crate::{container::ContainerRef, context::ViewContext};
#[cfg(feature = "log")]
mod logview;
#[cfg(feature = "log")]
pub use logview::LogPlugin;

pub trait Plugin {
    fn build(&mut self, _container: ContainerRef) {}
    fn before_render(&self, _ctx: &mut ViewContext, _container: ContainerRef) {}
    fn after_render(&self, _ctx: &mut ViewContext, _container: ContainerRef) {}
}
