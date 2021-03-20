mod backend;
mod format;
mod message;
mod syslog;
mod subscriber;

pub use format::{Facilities, Formatter3164, Severity, SyslogFormat};
pub use message::{ELKMessage, SyslogMessage};
pub use syslog::*;
pub use subscriber::{Subscriber, SubscriberBuilder};
pub use backend::LoggerBackendBuilder;

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
