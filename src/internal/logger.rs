use chrono::{DateTime, Utc, format};
use std::{time::SystemTime, collections::HashMap};

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
            LogLevel::Debug => write!(f,"👀 DEBUG"),
            LogLevel::Info => write!(f,"✅ INFO"),
            LogLevel::Warn => write!(f,"🚧 WARN"),
            LogLevel::Error => write!(f,"🙀 ERROR"),
            LogLevel::Critical => write!(f,"❗ CRITICAL"),
        }
    }
}


fn log<T: std::fmt::Display>(level: LogLevel, message: &T, context: Option<HashMap<&str, &str>>){
    let timestamp: DateTime<Utc> = SystemTime::now().into();
    
    
    let mut ctx_str = String::new();
    match context{
        Some(ctx) =>{
            
            for (key, value) in ctx{
                ctx_str.push_str(format!(" {}={}", key, value).as_str())
            }
            
        },
        None => (),
    }
    println!("{} |{:<10}| {} {}", timestamp.format("%Y-%m-%d %H:%M:%S.%3f"), level.to_string(), message, ctx_str);
    
}

pub fn debug<T: std::fmt::Display>(message: &T, context: Option<HashMap<&str, &str>>){
    log(LogLevel::Debug,&message, context)
}

pub fn info<T: std::fmt::Display>(message: &T, context: Option<HashMap<&str, &str>>){
    log(LogLevel::Info,&message, context)
}
pub fn warn<T: std::fmt::Display>(message: &T, context: Option<HashMap<&str, &str>>){
    log(LogLevel::Warn,&message, context)
}
pub fn error<T: std::fmt::Display>(message: &T, context: Option<HashMap<&str, &str>>){
    log(LogLevel::Error,&message, context)
}
pub fn critical<T: std::fmt::Display>(message: &T, context: Option<HashMap<&str, &str>>){
    log(LogLevel::Critical,&message, context)
}