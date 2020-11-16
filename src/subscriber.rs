use crate::{SyslogFormat, SyslogMessage, Formatter3164, ELKMessage};
use crate::backend::LoggerBackend;

type DefaultBuilder = SubscriberBuilder<Formatter3164, ELKMessage>;

/// Builder for subscriber.
#[derive(Debug)]
pub struct SubscriberBuilder<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    backend: LoggerBackend,
    header_formatter: F,
    message_formatter: M,
}

impl<F, M> SubscriberBuilder<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    pub fn set_header_formatter<E>(self, header_formatter: E) -> SubscriberBuilder<E, M>
    where
        E: SyslogFormat,
    {
        SubscriberBuilder {
            backend: self.backend,
            header_formatter,
            message_formatter: self.message_formatter,
        }
    }
    pub fn set_message_formatter<D>(self, message_formatter: D) -> SubscriberBuilder<F, D>
    where
        D: SyslogMessage,
    {
        SubscriberBuilder {
            backend: self.backend,
            header_formatter: self.header_formatter,
            message_formatter,
        }
    }
    pub fn set_backend(self, backend: LoggerBackend) -> SubscriberBuilder<F, M>
    {
        SubscriberBuilder {
            backend,
            header_formatter: self.header_formatter,
            message_formatter: self.message_formatter,
        }
    }
    pub fn finish(self) -> Subscriber<F, M> {
        Subscriber {
            backend: self.backend,
            header_formatter: self.header_formatter,
            message_formatter: self.message_formatter,
        }
    }
}

impl Default for DefaultBuilder {
    fn default() -> DefaultBuilder {
        let backend = LoggerBackend::default();
        let header_formatter = Formatter3164::default();
        let message_formatter = ELKMessage::default();
        Self { backend, header_formatter, message_formatter }
    }
}

/// Subscriber.
#[derive(Debug)]
pub struct Subscriber<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    pub backend: LoggerBackend,
    pub header_formatter: F,
    pub message_formatter: M,
}

impl<F, M> Subscriber<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    pub fn builder() -> DefaultBuilder {
        let backend = LoggerBackend::default();
        let header_formatter = Formatter3164::default();
        let message_formatter = ELKMessage::default();
        SubscriberBuilder { backend, header_formatter, message_formatter }
    }
}
