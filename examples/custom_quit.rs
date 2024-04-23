use arkham::prelude::*;

fn main() {
    App::new(root_view).disbale_q_to_quit().run().unwrap();
}

fn root_view(ctx: &mut ViewContext, kb: Res<Keyboard>) {
    ctx.insert((5, 2), "q does nothing, d quits");
    if kb.char() == Some('d') {
        ctx.exit();
    }
}
