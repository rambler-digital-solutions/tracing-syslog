use crate::backend::LoggerBackend;
use crate::{ELKMessage, Formatter3164, SyslogFormat, SyslogMessage};

/// Builder for layer.
#[derive(Debug)]
pub struct LayerBuilder<F = Formatter3164, M = ELKMessage>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    backend: LoggerBackend,
    header_formatter: F,
    message_formatter: M,
}

impl<F, M> LayerBuilder<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    pub fn set_header_formatter<E>(self, header_formatter: E) -> LayerBuilder<E, M>
    where
        E: SyslogFormat,
    {
        LayerBuilder {
            backend: self.backend,
            header_formatter,
            message_formatter: self.message_formatter,
        }
    }
    pub fn set_message_formatter<D>(self, message_formatter: D) -> LayerBuilder<F, D>
    where
        D: SyslogMessage,
    {
        LayerBuilder {
            backend: self.backend,
            header_formatter: self.header_formatter,
            message_formatter,
        }
    }
    pub fn set_backend(self, backend: LoggerBackend) -> LayerBuilder<F, M> {
        LayerBuilder {
            backend,
            header_formatter: self.header_formatter,
            message_formatter: self.message_formatter,
        }
    }
    pub fn finish(self) -> Layer<F, M> {
        Layer {
            backend: self.backend,
            header_formatter: self.header_formatter,
            message_formatter: self.message_formatter,
        }
    }
}

impl Default for LayerBuilder {
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

/// Syslog Layer.
#[derive(Debug)]
pub struct Layer<F, M>
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
    pub fn builder() -> LayerBuilder<Formatter3164, ELKMessage> {
        LayerBuilder::default()
    }
}
