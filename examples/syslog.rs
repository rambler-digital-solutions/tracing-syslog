use tracing_subscriber::prelude::*;
use tracing::{event, Level};
use tracing_syslog::{Formatter3164, ELKMessage};

fn main() {
    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::subscriber().with_target(false));
    let subscriber = tracing_syslog::Subscriber::<Formatter3164, ELKMessage>::builder().finish();
    registry.with(subscriber).init();
    event!(Level::INFO, key_one = "value_one", key_two = "value_two");
}
