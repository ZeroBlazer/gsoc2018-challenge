pub trait LoggerTypes {
    type LogEntryType: LogEntryTypes;

    fn new() -> Self;

    fn log(&self, s: &str) -> Self::LogEntryType;
}

pub trait LogEntryTypes: AsRef<str> {
    fn new(s: &str) -> Self;
}