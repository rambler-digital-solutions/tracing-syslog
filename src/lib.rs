mod backend;
mod format;
mod layer;
mod message;
mod syslog;

pub use backend::LoggerBackendBuilder;
pub use format::{Facilities, Formatter3164, Severity, SyslogFormat};
pub use layer::Layer;
pub use message::{ELKMessage, SyslogMessage};
pub use syslog::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HOSTNAME: String = {
        match std::env::var("HOSTNAME") {
            Ok(value) => value,
            Err(_) => "undefined".to_string(),
        }
    };
}
