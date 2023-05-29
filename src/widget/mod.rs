use crate::prelude::{Res, Runes, Theme, ViewContext};

pub trait Widget {
    fn ui(&mut self, ctx: &mut ViewContext);
}

fn list(items: Vec<Runes>, selection_index: usize) -> impl FnOnce(&mut ViewContext, Res<Theme>) {
    move |ctx: &mut ViewContext, theme: Res<Theme>| {
        for (idx, item) in items.into_iter().enumerate() {
            ctx.insert((0, idx), item.clone());
        }
    }
}
