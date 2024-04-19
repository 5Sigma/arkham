use arkham::prelude::*;

fn main() {
    let _ = App::new(root).run();
}

fn root(ctx: &mut ViewContext) {
    let mut stack = ctx.vertical_stack((100, 100));
    for _ in 0..10 {
        stack.component((ctx.size().width, 1), list_item);
    }
    ctx.component((0, (100, 100)), stack);
}

fn list_item(ctx: &mut ViewContext) {
    let size = ctx.size();
    let mut hstack = ctx.horizontal_stack((ctx.size().width, 1));
    hstack.insert("> ");
    hstack.insert("line 1");
    ctx.component(size, hstack);
}
