use crate::{SyslogFormat, SyslogMessage, Formatter3164, ELKMessage};
use crate::backend::LoggerBackend;

type DefaultBuilder = SSubscriberBuilder<Formatter3164, ELKMessage>;
type DefaultSubscriber = SSubscriber<Formatter3164, ELKMessage>;

/// Builder for subscriber.
#[derive(Debug)]
pub struct SSubscriberBuilder<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    backend: LoggerBackend,
    formatter: F,
    message: M,
}

impl<F, M> SSubscriberBuilder<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    fn set_header_formatter<E>(self, formatter: E) -> SSubscriberBuilder<E, M>
    where
        E: SyslogFormat,
    {
        SSubscriberBuilder {
            formatter,
            backend: self.backend,
            message: self.message,
        }
    }
    fn set_message_formatter<D>(self, message: D) -> SSubscriberBuilder<F, D>
    where
        D: SyslogMessage,
    {
        SSubscriberBuilder {
            formatter: self.formatter,
            backend: self.backend,
            message,
        }
    }
    fn set_backend(self, backend: LoggerBackend) -> SSubscriberBuilder<F, M>
    {
        SSubscriberBuilder {
            formatter: self.formatter,
            backend,
            message: self.message,
        }
    }
    fn finish(self) -> SSubscriber<F, M> {
        SSubscriber {
            formatter: self.formatter,
            backend: self.backend,
            message: self.message,
        }
    }
}

impl Default for DefaultBuilder {
    fn default() -> DefaultBuilder {
        let backend = LoggerBackend::default();
        let formatter = Formatter3164::default();
        let message = ELKMessage::default();
        Self { backend, formatter, message }
    }
}

/// Subscriber.
#[derive(Debug)]
pub struct SSubscriber<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    backend: LoggerBackend,
    formatter: F,
    message: M,
}

impl<F, M> SSubscriber<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    pub fn builder() -> DefaultBuilder {
        let backend = LoggerBackend::default();
        let formatter = Formatter3164::default();
        let message = ELKMessage::default();
        SSubscriberBuilder { backend, formatter, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Formatter3164, ELKMessage};

    #[test]
    fn test_one() {
        let subscriber = SSubscriber::builder()
            .set_backend(LoggerBackend::default())
            .set_header_formatter(Formatter3164::default())
            .set_message_formatter(ELKMessage::default())
            .finish();
        assert_eq!(1, 1);
    }
}
