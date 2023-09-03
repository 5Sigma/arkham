use arkham::prelude::*;

#[derive(Copy, Clone)]
enum Route {
    Main,
    Secondary,
}

fn main() {
    let _ = App::new(root).insert_state(Route::Main).run();
}

fn root(ctx: &mut ViewContext, route: State<Route>) {
    let size = ctx.size();
    let r = *route.get();
    ctx.insert((0, 0), "Press Q to quit");
    match r {
        Route::Main => {
            ctx.component(Rect::new((0, 1), size), main_route);
        }
        Route::Secondary => {
            ctx.component(Rect::new((0, 1), size), secondary_route);
        }
    }
}

fn main_route(ctx: &mut ViewContext, kb: Res<Keyboard>, route: State<Route>) {
    ctx.insert(
        0,
        "Welcome to the main screen, press 2 to goto the secondary screen",
    );
    if kb.char() == Some('2') {
        *route.get_mut() = Route::Secondary;
        ctx.render();
    }
}

fn secondary_route(ctx: &mut ViewContext, kb: Res<Keyboard>, route: State<Route>) {
    ctx.insert(
        0,
        "Welcome to the secondary screen, press 1 to goto the main screen",
    );

    if kb.char() == Some('1') {
        *route.get_mut() = Route::Main;
        ctx.render();
    }
}
