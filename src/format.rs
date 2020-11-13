use std::fmt::Display;

use chrono::Local;

pub trait SyslogFormat {
    fn format<T: Display>(&self, message: T) -> String;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Informational = 6,
    Debug = 7,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Facilities {
    Local0 = 16,
    Local1 = 17,
    Local2 = 18,
    Local3 = 19,
    Local4 = 20,
    Local5 = 21,
    Local6 = 22,
    Local7 = 23,
}

#[derive(Debug)]
pub struct Formatter3164 {
    pub facility: Facilities,
    pub severity: Severity,
    pub hostname: String,
    pub pid: u32,
}

impl Formatter3164 {
    pub fn priority(&self) -> u8 {
        self.facility as u8 * 8 + self.severity as u8
    }
}

impl Default for Formatter3164 {
    fn default() -> Self {
        Self {
            facility: Facilities::Local0,
            severity: Severity::Debug,
            hostname: String::from("hostname"),
            pid: 1,
        }
    }
}

impl SyslogFormat for Formatter3164 {
    fn format<T: Display>(&self, message: T) -> String {
        let dt = Local::now();
        format!(
            "<{pri}>{date} {hostname} [{pid}]: {message}",
            pri = self.priority(),
            date = dt.format("%b %d %T"),
            hostname = self.hostname,
            pid = self.pid,
            message = message,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        let formatter = Formatter3164 {
            facility: Facilities::Local0,
            severity: Severity::Alert,
            hostname: String::from("Host"),
            pid: 1,
        };
        let pri = formatter.priority();
        assert_eq!(pri, 129);
    }

    #[test]
    fn test_display() {
        let formatter = Formatter3164 {
            facility: Facilities::Local0,
            severity: Severity::Alert,
            hostname: String::from("Host"),
            pid: 1,
        };
        let syslog_message = formatter.format("message");
        assert_ne!(
            syslog_message,
            String::from("<129>Nov 12 17:14:05 Host [1]: message")
        );
    }
}
