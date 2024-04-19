use arkham::prelude::*;

#[derive(Default)]
pub struct AppState {
    pub counter: i32,
}

fn main() {
    let app_state = State::new(AppState::default());
    let mut app = App::new(root_view).bind_state(app_state.clone());
    let renderer = app.get_renderer();

    std::thread::spawn(move || loop {
        app_state.get_mut().counter += 1;
        renderer.render();
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    app.run().unwrap();
}

fn root_view(ctx: &mut ViewContext, state: State<AppState>) {
    ctx.insert(0, format!("Count is {}", state.get().counter));
}
