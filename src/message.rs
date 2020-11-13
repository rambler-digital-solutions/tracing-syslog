pub trait SyslogMessage {
    fn message(&self, pairs: Vec<(String, String)>) -> String;
}

#[derive(Debug)]
pub struct ELKMessage;

impl Default for ELKMessage {
    fn default() -> Self {
        ELKMessage
    }
}

impl SyslogMessage for ELKMessage {
    fn message(&self, pairs: Vec<(String, String)>) -> String {
        let formatted = pairs
            .iter()
            .map(|(key, value)| format!("{:?}: {}", key, value))
            .collect::<Vec<_>>()
            .join(", ");
        let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%z").to_string();
        format!(
            "{{{:?}: {:?}, {:?}: {:?}, {}}}",
            "@timestamp", now,
            "hostname", "hostname",
            formatted,
        )
    }
}
