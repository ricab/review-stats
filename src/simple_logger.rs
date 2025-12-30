use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::Write;

pub struct SimpleLogger {
    level: LevelFilter,
}

impl SimpleLogger {
    pub fn init(level: LevelFilter) {
        let logger = Box::new(SimpleLogger { level });
        log::set_max_level(level);
        log::set_logger(Box::leak(logger)).expect("Failed to set logger");
    }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if record.level() <= Level::Warn {
                eprintln!("{}: {}", record.level(), record.args());
            } else {
                println!("{}: {}", record.level(), record.args());
            }
        }
    }

    fn flush(&self) {
        std::io::stdout().flush().ok();
        std::io::stderr().flush().ok();
    }
}
