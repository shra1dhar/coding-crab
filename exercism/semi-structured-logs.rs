/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("[{}]: {}", level, message);
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message);
}
pub fn warn(message: &str) -> String {
    log(LogLevel::WARNING, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}