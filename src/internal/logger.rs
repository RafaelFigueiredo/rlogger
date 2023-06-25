use chrono::{DateTime, Utc};
use std::time::SystemTime;

enum LogLevel{
    Debug,
    Info,
    Warn,
    Error,
    Critical
}



impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f,"debug"),
            LogLevel::Info => write!(f,"info"),
            LogLevel::Warn => write!(f,"warn"),
            LogLevel::Error => write!(f,"error"),
            LogLevel::Critical => write!(f,"critical"),   
        }
    }
}


fn log<T: std::fmt::Display>(level: LogLevel, message: &T){
    let timestamp: DateTime<Utc> = SystemTime::now().into();
    let timestamp = timestamp.to_rfc3339();

    println!("{}  {}  {}", timestamp, level, message)
}

pub fn debug<T: std::fmt::Display>(message: &T){
    log(LogLevel::Debug,&message)
}

pub fn info<T: std::fmt::Display>(message: &T){
    log(LogLevel::Info,&message)
}
pub fn warn<T: std::fmt::Display>(message: &T){
    log(LogLevel::Warn,&message)
}
pub fn error<T: std::fmt::Display>(message: &T){
    log(LogLevel::Error,&message)
}
pub fn critical<T: std::fmt::Display>(message: &T){
    log(LogLevel::Critical,&message)
}