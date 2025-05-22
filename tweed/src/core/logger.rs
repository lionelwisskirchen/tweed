#[cfg(feature = "std")]
use log::{LevelFilter, Log, Record};

pub struct STDLogger;
#[cfg(feature = "std")]
impl Log for STDLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
#[cfg(feature = "std")]
static LOGGER: STDLogger = STDLogger;
#[cfg(feature = "std")]
impl STDLogger {
    pub fn init(level: LevelFilter) {
        let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(level));
    }
}
