// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}

pub fn info(message: &str) -> String {
    let mut owned_string: String = "[INFO]: ".to_owned();
    owned_string.push_str(message);
    return owned_string;
}

pub fn warn(message: &str) -> String {
    let mut owned_string: String = "[WARNING]: ".to_owned();
    owned_string.push_str(message);
    return owned_string;
}

pub fn error(message: &str) -> String {
    let mut owned_string: String = "[ERROR]: ".to_owned();
    owned_string.push_str(message);
    return owned_string;
}
