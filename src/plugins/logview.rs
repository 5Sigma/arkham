use crate::{container::ContainerRef, plugins::Plugin, prelude::*};
use log::{Level, LevelFilter, Metadata, Record};
use std::collections::VecDeque;
use std::sync::atomic::AtomicUsize;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

#[derive(Default)]
pub struct LogViewState {}

pub struct LogRecord {
    pub level: Level,
    pub message: String,
    pub time: chrono::DateTime<chrono::Local>,
}

#[derive(Default, Clone)]
pub struct ArkhamLogger {
    records: Arc<Mutex<VecDeque<LogRecord>>>,
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
            let mut rcs = self.records.lock().unwrap();
            rcs.push_back(LogRecord {
                level: record.level(),
                message: record.args().to_string(),
                time: chrono::Local::now(),
            });
            if rcs.len() > 500 {
                rcs.pop_front();
            }
        }
    }

    fn flush(&self) {}
}

pub struct LogPlugin {
    logger: &'static ArkhamLogger,
    log_open: AtomicBool,
    offset: AtomicUsize,
    locked: AtomicBool,
}

impl Default for LogPlugin {
    fn default() -> Self {
        Self {
            logger: ArkhamLogger::setup().unwrap(),
            log_open: AtomicBool::new(false),
            offset: AtomicUsize::new(0),
            locked: AtomicBool::new(true),
        }
    }
}

impl Plugin for LogPlugin {
    fn build(&mut self, container: ContainerRef) {
        let _ = log::set_logger(self.logger);
        container.borrow_mut().bind(Res::new(self.logger));
    }

    fn before_render(&self, _ctx: &mut ViewContext, args: ContainerRef) {
        let args = args.borrow();
        let kb = args.get::<Res<Keyboard>>().unwrap();
        let mut open = self.log_open.load(std::sync::atomic::Ordering::SeqCst);
        if kb.char() == Some('~') {
            open = !open;
            self.locked.store(true, std::sync::atomic::Ordering::SeqCst);
            self.log_open
                .store(open, std::sync::atomic::Ordering::SeqCst);
            kb.reset();
        }

        if open {
            if kb.char() == Some('j') || kb.code() == Some(KeyCode::Down) {
                self.locked
                    .store(false, std::sync::atomic::Ordering::SeqCst);
                let offset = self.offset.load(std::sync::atomic::Ordering::SeqCst);
                if offset < self.logger.records.lock().unwrap().len() - 1 {
                    self.offset
                        .store(offset + 1, std::sync::atomic::Ordering::SeqCst);
                }
                kb.reset();
            }

            if kb.char() == Some('k') || kb.code() == Some(KeyCode::Up) {
                self.locked
                    .store(false, std::sync::atomic::Ordering::SeqCst);
                let offset = self.offset.load(std::sync::atomic::Ordering::SeqCst);
                if offset > 0 {
                    self.offset
                        .store(offset - 1, std::sync::atomic::Ordering::SeqCst);
                }
                kb.reset();
            }
        }
    }

    fn after_render(&self, ctx: &mut ViewContext, _args: ContainerRef) {
        let len = self.logger.records.lock().unwrap().len();
        let height = ctx.height() - 2;
        if self.locked.load(std::sync::atomic::Ordering::SeqCst) {
            if len > height {
                self.offset
                    .store(len - height, std::sync::atomic::Ordering::SeqCst);
            } else {
                self.offset.store(0, std::sync::atomic::Ordering::SeqCst);
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
            ctx.insert(0, "  Log view".to_runes().bold());
            ctx.component(
                ((0, 2), size - Size::new(0, 2)),
                logview(self.offset.load(std::sync::atomic::Ordering::SeqCst)),
            );
        }
    }
}

fn logview(offset: usize) -> impl Fn(&mut ViewContext, Res<&ArkhamLogger>) {
    move |ctx: &mut ViewContext, logger: Res<&ArkhamLogger>| {
        let records = logger.records.lock().unwrap();
        for (idx, entry) in records.iter().skip(offset).enumerate() {
            ctx.component(((2, idx), (6, 1)), level(entry.level));
            ctx.insert(
                (9, idx),
                entry
                    .time
                    .format("%H:%M:%S")
                    .to_string()
                    .to_runes()
                    .fg(Color::DarkGrey),
            );
            ctx.insert((18, idx), entry.message.clone());
        }
    }
}

fn level(level: Level) -> impl Fn(&mut ViewContext) {
    let bg = match level {
        Level::Error => Color::Rgb { r: 110, g: 0, b: 0 },
        Level::Warn => Color::Rgb {
            r: 110,
            g: 105,
            b: 24,
        },
        Level::Info => Color::Rgb {
            r: 255,
            g: 255,
            b: 255,
        },
        Level::Debug => Color::Rgb { r: 0, g: 0, b: 0 },
        Level::Trace => Color::Rgb { r: 0, g: 0, b: 0 },
    };

    let fg = match level {
        Level::Info => Color::Rgb { r: 0, g: 0, b: 0 },
        _ => Color::Rgb {
            r: 255,
            g: 255,
            b: 255,
        },
    };
    move |ctx| {
        ctx.fill_all(bg);
        ctx.insert(0, level.to_string().to_runes().fg(fg).bold())
    }
}
