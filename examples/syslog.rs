use tracing_subscriber::prelude::*;
use tracing::{event, Level};
use tracing_syslog::{Formatter3164, ELKMessage};

fn main() {
    //let syslog_layer = tracing_syslog::Layer::<Formatter3164, ELKMessage>::builder().finish();
    let syslog_layer = tracing_syslog::Layer::builder().finish();
    tracing_subscriber::registry()
        .with(syslog_layer)
        .init();
    event!(Level::INFO, key_one = "value_one", key_two = "value_two");
}
