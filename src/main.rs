
mod internal;
use internal::logger;
fn main(){
    logger::debug(&"hello from debug");
    logger::info(&"hello from info");
    logger::warn(&"hello from warn");
    logger::error(&"hello from error");
    logger::critical(&"hello from critical");
}