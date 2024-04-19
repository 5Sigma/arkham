use arkham::prelude::*;

fn main() {
    App::new(root)
        .insert_resource(Theme::default())
        .run()
        .expect("couldnt launch app");
}

fn root(ctx: &mut ViewContext, theme: Res<Theme>) {
    let size = ctx.size();
    ctx.fill_all(theme.bg_primary);
    ctx.fill(Rect::new((5, 5), size - 10), theme.bg_secondary);
    ctx.insert((10, 10), "Hello World");
    ctx.insert(
        ((size.width / 2) - 7, 0),
        "Press Q to Quit".to_runes().fg(theme.fg),
    );
}
