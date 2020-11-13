use tracing_subscriber::prelude::*;
use tracing_syslog;
use tracing::{event, Level};

fn main() {
    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::subscriber().with_target(false));
    let subscriber = tracing_syslog::SubscriberBuilder::default().finish();
    registry.with(subscriber).init();
    event!(Level::INFO, key_one = "value_one", key_two = "value_two");
}
