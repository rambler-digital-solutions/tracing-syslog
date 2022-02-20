use crate::backend::LoggerBackend;
use crate::{ELKMessage, Formatter3164, SyslogFormat, SyslogMessage};

/// A [`Layer`] that logs formatted representations of `tracing` events to syslog.
///
/// ## Examples
///
/// Constructing a layer with the default configuration:
///
/// ```rust
/// use tracing_subscriber::Registry;
/// use tracing_subscriber::prelude::*;
///
/// let subscriber = Registry::default()
///     .with(tracing_syslog::Layer::default());
///
/// tracing::subscriber::set_global_default(subscriber).unwrap();
/// ```
/// [`Layer`]: tracing_subscriber::Layer
#[derive(Debug)]
pub struct Layer<F = Formatter3164, M = ELKMessage>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    pub backend: LoggerBackend,
    pub header_formatter: F,
    pub message_formatter: M,
}

impl<F, M> Layer<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    pub fn set_header_formatter<E>(self, header_formatter: E) -> Layer<E, M>
    where
        E: SyslogFormat,
    {
        Layer {
            backend: self.backend,
            header_formatter,
            message_formatter: self.message_formatter,
        }
    }
    pub fn set_message_formatter<D>(self, message_formatter: D) -> Layer<F, D>
    where
        D: SyslogMessage,
    {
        Layer {
            backend: self.backend,
            header_formatter: self.header_formatter,
            message_formatter,
        }
    }
    pub fn set_backend(self, backend: LoggerBackend) -> Layer<F, M> {
        Layer {
            backend,
            header_formatter: self.header_formatter,
            message_formatter: self.message_formatter,
        }
    }
}

impl Default for Layer {
    fn default() -> Self {
        let backend = LoggerBackend::default();
        let header_formatter = Formatter3164::default();
        let message_formatter = ELKMessage::default();
        Self {
            backend,
            header_formatter,
            message_formatter,
        }
    }
}
