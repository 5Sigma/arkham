use crate::{container::ContainerRef, plugins::Plugin, prelude::*};
use log::{Level, LevelFilter, Metadata, Record};
use std::sync::{atomic::AtomicBool, Arc, Mutex};

pub struct LogRecord {
    pub level: Level,
    pub message: String,
}

#[derive(Default, Clone)]
pub struct ArkhamLogger {
    records: Arc<Mutex<Vec<LogRecord>>>,
}

impl ArkhamLogger {
    pub(crate) fn setup() -> anyhow::Result<&'static Self> {
        let logger = Box::leak(Box::new(Self::default()));
        let _ = log::set_logger(logger);
        log::set_max_level(LevelFilter::Info);
        Ok(logger)
    }
}

impl log::Log for ArkhamLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.records.lock().unwrap().push(LogRecord {
                level: record.level(),
                message: record.args().to_string(),
            });
        }
    }

    fn flush(&self) {}
}

pub struct LogPlugin {
    logger: &'static ArkhamLogger,
    log_open: AtomicBool,
}

impl Default for LogPlugin {
    fn default() -> Self {
        Self {
            logger: ArkhamLogger::setup().unwrap(),
            log_open: AtomicBool::new(false),
        }
    }
}

impl Plugin for LogPlugin {
    fn build(&mut self, container: ContainerRef) {
        let _ = log::set_logger(self.logger);
        container.borrow_mut().bind(Res::new(self.logger));
    }

    fn before_render(&self, _ctx: &mut ViewContext, _args: ContainerRef) {}

    fn after_render(&self, ctx: &mut ViewContext, args: ContainerRef) {
        let c = args.borrow();
        let kb = c.get::<Res<Keyboard>>().unwrap();
        if kb.char() == Some('~') {
            drop(c);
            if self.log_open.load(std::sync::atomic::Ordering::SeqCst) {
                self.log_open
                    .store(false, std::sync::atomic::Ordering::SeqCst);
            } else {
                self.log_open
                    .store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }
        if self.log_open.load(std::sync::atomic::Ordering::SeqCst) {
            let size = ctx.size();
            ctx.fill_all(Color::Rgb { r: 0, g: 0, b: 0 });
            ctx.fill(
                ((0, 0), (size.width, 1)),
                Color::Rgb {
                    r: 30,
                    g: 30,
                    b: 30,
                },
            );
            ctx.insert(0, "  Log view");
            ctx.component(((0, 2), size - Size::new(0, 2)), logview);
        }
    }
}

fn logview(ctx: &mut ViewContext, logger: Res<&ArkhamLogger>) {
    let records = logger.records.lock().unwrap();
    for (idx, entry) in records.iter().enumerate() {
        ctx.component(((0, idx), (6, 1)), level(entry.level));
        ctx.insert((7, idx), entry.message.clone());
    }
}

fn level(level: Level) -> impl Fn(&mut ViewContext) {
    let color = match level {
        Level::Error => Color::Rgb { r: 110, g: 0, b: 0 },
        Level::Warn => Color::Rgb {
            r: 110,
            g: 105,
            b: 24,
        },
        Level::Info => Color::Rgb { r: 0, g: 0, b: 0 },
        Level::Debug => Color::Rgb { r: 0, g: 0, b: 0 },
        Level::Trace => Color::Rgb { r: 0, g: 0, b: 0 },
    };
    move |ctx| {
        ctx.fill_all(color);
        ctx.insert(0, level.to_string())
    }
}
