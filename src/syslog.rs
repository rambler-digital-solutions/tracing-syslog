use std::cmp::Ordering;
use std::fmt;

use tracing_core::{event::Event, field::Visit, Collect, Field, Level};
use tracing_subscriber::{registry::LookupSpan, subscribe::Context};

use crate::backend::LoggerBackend;
use crate::{ELKMessage, Formatter3164, SyslogFormat, SyslogMessage};


/// Builder for subscriber.
#[derive(Debug)]
pub struct SubscriberBuilder<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    backend: LoggerBackend,
    formatter: F,
    message: M,
}

impl Default for SubscriberBuilder<Formatter3164, ELKMessage> {
    fn default() -> Self {
        let backend = LoggerBackend::default();
        let formatter = Formatter3164::default();
        let message = ELKMessage::default();
        Self { backend, formatter, message }
    }
}

impl<F, M> SubscriberBuilder<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    pub fn set_formatter<E>(self, formatter: E) -> SubscriberBuilder<E, M>
    where
        E: SyslogFormat,
    {
        SubscriberBuilder {
            formatter,
            backend: self.backend,
            message: self.message,
        }
    }
    pub fn finish(self) -> Subscriber<F, M> {
        Subscriber {
            backend: self.backend,
            message: self.message,
            formatter: self.formatter,
        }
    }
}

#[derive(Debug)]
pub struct Subscriber<F, M>
where
    F: SyslogFormat,
    M: SyslogMessage,
{
    backend: LoggerBackend,
    formatter: F,
    message: M,
}

impl Default for Subscriber<Formatter3164, ELKMessage>
{
    fn default() -> Self {
        let backend = LoggerBackend::default();
        let formatter = Formatter3164::default();
        let message = ELKMessage::default();
        Self { backend, formatter, message }
    }
}

impl<C, F, M> tracing_subscriber::Subscribe<C> for Subscriber<F, M>
where
    C: Collect + for<'span> LookupSpan<'span>,
    F: 'static + SyslogFormat,
    M: 'static + SyslogMessage,
{
    fn on_event(&self, event: &Event, _: Context<C>) {
        let mut pairs: Vec<(String, String)> = Vec::with_capacity(256);
        if let Ordering::Less = event.metadata().level().cmp(&Level::TRACE) {
            event.record(&mut EventVisitor::new(&mut pairs));
            let message = self.message.message(pairs);
            let message = self.formatter.format(message);
            let _ = &self.backend.send(&message.as_bytes());
        }
    }
}

#[derive(Debug)]
struct EventVisitor<'a> {
    pairs: &'a mut Vec<(String, String)>,
}

impl<'a> EventVisitor<'a> {
    fn new(pairs: &'a mut Vec<(String, String)>) -> Self {
        Self { pairs }
    }
}

impl Visit for EventVisitor<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        let key = field.name().to_string();
        let value = format!("{:?}", value);
        self.pairs.push((key, value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let _subscriber = Subscriber::default();
        assert_eq!(1, 1);
    }
}
