use std::cmp::Ordering;
use std::fmt;

use tracing_core::{Collect, event::Event, Field, field::Visit, Level};
use tracing_subscriber::{registry::LookupSpan, subscribe::Context};

use crate::{Subscriber, SyslogFormat, SyslogMessage};

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
            let message = self.message_formatter.message(pairs);
            let message = self.header_formatter.format(message);
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
