use std::cmp::Ordering;
use tracing_core::{
    event::Event,
    field::{Field, Visit},
    metadata::Level,
    Subscriber,
};
use tracing_subscriber::{layer::Context, registry::LookupSpan};

use crate::{Layer, SyslogFormat, SyslogMessage};

impl<S, F, M> tracing_subscriber::Layer<S> for Layer<F, M>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    F: 'static + SyslogFormat,
    M: 'static + SyslogMessage,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut pairs: Vec<(String, String)> = Vec::with_capacity(256);
        if let Ordering::Less = event.metadata().level().cmp(&Level::TRACE) {
            event.record(&mut EventVisitor::new(&mut pairs));
            let message = self.message_formatter.message(pairs);
            let message = self.header_formatter.format(message);
            let _ = &self.backend.send(message.as_bytes());
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
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        let key = field.name().to_string();
        let value = format!("{:?}", value);
        self.pairs.push((key, value));
    }
}
