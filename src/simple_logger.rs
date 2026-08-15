use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io::{self, Write};

pub struct SimpleLogger {
    level: LevelFilter,
}

impl SimpleLogger {
    pub fn init(level: LevelFilter) {
        let logger = Box::new(SimpleLogger { level });
        log::set_max_level(level);
        log::set_logger(Box::leak(logger)).expect("Failed to set logger");
    }

    fn target(&self, level: Level) -> Box<dyn Write> {
        if level > self.level {
            Box::new(io::sink())
        } else if level <= Level::Warn {
            Box::new(io::stderr())
        } else {
            Box::new(io::stdout())
        }
    }
}

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let mut target = self.target(record.level());
        writeln!(target, "{}: {}", record.level(), record.args()).ok();
    }

    fn flush(&self) {
        io::stdout().flush().ok();
        io::stderr().flush().ok();
    }
}
