use arkham::prelude::*;

fn main() {
    let mut app = App::new(root_view).insert_plugin(arkham::plugins::LogPlugin::default());

    log::trace!("Test trace message");
    log::debug!("Test debug message");
    log::info!("Test info message");
    log::warn!("Test warning message");
    log::error!("Test error message");

    let _ = app.run();
}

fn root_view(ctx: &mut ViewContext, kb: Res<Keyboard>) {
    ctx.fill_all(Color::Black);
    ctx.insert((2, 2), "Press any key to register a log entry".to_runes());
    ctx.insert((2, 3), "Press ~ to view the log");
    ctx.insert((2, 4), "Press q quit");

    if let Some(c) = kb.char() {
        log::info!("Key was pressed: {}", c);
    }
}
