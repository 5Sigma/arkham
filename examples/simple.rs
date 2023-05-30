use arkham::prelude::*;

fn main() {
    App::new(root).run().expect("couldnt launch app");
}

fn root(ctx: &mut ViewContext) {
    let size = ctx.size();
    ctx.fill(size, Rune::new().bg(Color::DarkGrey));
    ctx.insert((10, 10), "Hello World");
    ctx.insert(
        ((size.width / 2) - 7, 0),
        "Press Q to Quit".to_runes().fg(Color::Red),
    );
}
