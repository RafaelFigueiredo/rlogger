
mod internal;
use internal::logger;
fn main(){
    logger::debug(&"hello from debug", None);
    logger::info(&"hello from info", None);
    logger::warn(&"hello from warn", None);
    logger::error(&"hello from error", None);
    logger::critical(&"hello from critical", None);
}