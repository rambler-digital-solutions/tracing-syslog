use crate::HOSTNAME;

pub trait SyslogMessage {
    fn message(&self, pairs: Vec<(String, String)>) -> String;
}

#[derive(Debug, Default)]
pub struct ELKMessage;

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
            "hostname", HOSTNAME.to_string(),
            formatted,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostname() {
        std::env::set_var("HOSTNAME", "test_hostname".to_string());
        let message = ELKMessage.message(vec![]);
        assert!(message.contains("\"hostname\": \"test_hostname\""))
    }

    #[test]
    fn test_pairs() {
        let pairs = vec![("key_1".to_string(), "value_1".to_string())];
        let message = ELKMessage.message(pairs);
        assert!(message.contains("\"key_1\": value_1"))
    }
}
