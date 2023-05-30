use arkham::prelude::*;

fn main() {
    App::new(root).run().expect("couldnt launch app");
}

fn root(ctx: &mut ViewContext) {
    let size = ctx.size();
    ctx.fill(size, Rune::new().bg(Color::DarkGrey));
    ctx.component(Rect::new((10, 10), (20, 1)), say_hello("Alice"));
    ctx.component(Rect::new(0, (size.width, 1)), quit_nag);
}

fn say_hello(name: &'static str) -> impl Fn(&mut ViewContext) {
    move |ctx: &mut ViewContext| {
        ctx.insert((0, 0), format!("Hello, {}", name));
    }
}

fn quit_nag(ctx: &mut ViewContext) {
    let size = ctx.size();
    ctx.insert(
        ((size.width / 2) - 7, 0),
        "Press Q to Quit".to_runes().fg(Color::Red),
    );
}
