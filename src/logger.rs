use std::sync::Mutex;
use std::fs;
use std::io::Write;
extern crate time;
extern crate log;
use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter};

pub struct Logger {
    log: Mutex<fs::File>
}

impl Logger {
    pub fn new(file: fs::File) -> Box<Logger> {
        Box::new(Logger { log: Mutex::new(file) })
    }

    pub fn init() {
        log::set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Debug);
            Logger::new(fs::OpenOptions::new().create(true).write(true).open("log.txt").unwrap())
        }).unwrap()
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Debug
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let mut log_lock = self.log.lock().unwrap();

            let cur_time = time::now();
            writeln!(log_lock,
                    "[{:}]  {}",
                    cur_time.rfc822(),
                    record.args()).unwrap();
        }
    }
}
