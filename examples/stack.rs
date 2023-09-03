use arkham::prelude::*;

fn main() {
    let _ = App::new(root).run();
}

fn root(ctx: &mut ViewContext) {
    let mut stack = ctx.vertical_stack(Size::new(100, 100));
    for _ in 0..10 {
        stack.component(Size::new(ctx.size().width, 2), list_item);
    }
    ctx.component(Rect::new(0, (100, 100)), stack);
}

fn list_item(ctx: &mut ViewContext) {
    ctx.insert(0, "line 1");
}
