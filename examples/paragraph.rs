use arkham::prelude::*;

fn main() {
    let _ = App::new(root).run();
}

fn root(ctx: &mut ViewContext) {
    let mut stack = ctx.vertical_stack(ctx.size());
    let p = arkham::components::Paragraph::new("Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety—ensuring that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages.");
    stack.component(Size::new(100_usize, p.height(100)), p);
    let p = arkham::components::Paragraph::new("Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety—ensuring that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages.");
    stack.component(Size::new(100_usize, p.height(100)), p);
    ctx.component(ctx.size(), stack);
}
