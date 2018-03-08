// extern crate base-api as base; // Might be confusing
extern crate base_api;

use base_api::*;
use std::mem;

pub struct Middleware<LoggerType: LoggerTypes, LogEntryType: LogEntryTypes> {
    logger: LoggerType,
    violations: Vec<LogEntryType>,
}

impl<LoggerType: LoggerTypes, LogEntryType: LogEntryTypes> Middleware<LoggerType, LogEntryType> {
    fn new(logger: LoggerType) -> Middleware<LoggerType, LogEntryType> {
        Middleware {
            logger,
            violations: vec![]
        }
    }

    pub fn log_violation(&mut self, s: &str) {
        self.violations.push(self.logger.log(s));
    }

    pub fn take_violations(&mut self) -> Vec<LogEntryType> {
        mem::replace(&mut self.violations, vec![])
    }

    pub fn take_logger(self) -> LoggerType {
        self.logger
    }
}

pub fn create_middleware<LoggerType: LoggerTypes, LogEntryType: LogEntryTypes>() -> Middleware<LoggerType, LogEntryType> {
    Middleware::new(LoggerType::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut m = create_middleware();
        m.log_violation("hi");
        m.log_violation("byte");
        let vs: Vec<_> = m.take_violations().into_iter().map(|v| v.as_ref()).collect();
        assert_eq!(&vs, &["hi", "bye"]);
    }
}
