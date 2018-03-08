extern crate base_api;

use std::convert::AsRef;

use base_api::*;
// use base_api::{Logger, LogEntry};
// use base_api::{LoggerTypes, LogEntryTypes};

pub struct Logger;

pub struct LogEntry(String);


impl LogEntryTypes for LogEntry {
    fn new(s: &str) -> LogEntry {
        LogEntry(s.to_string())
    }
}

impl AsRef<str> for LogEntry {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl LoggerTypes for Logger {
    type LogEntryType = LogEntry;

    fn new() -> Logger {
        Logger
    }
    
    fn log(&self, s: &str) -> LogEntry {
        LogEntry::new(s)
    }
}

pub fn do_a_bunch_of_things(logger: &Logger, things: &[&str]) -> Vec<LogEntry> {
    things.iter().map(|s| logger.log(s)).collect()
}
