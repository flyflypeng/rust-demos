use tracing::{debug, error};
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, Registry,
};

mod a {
    pub mod b {
        pub mod c {
            use tracing::info;

            pub fn hello() {
                info!("Info JPF!");
            }
        }
    }
}

fn main() {
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    let log_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_target(true)
        .with_level(true)
        .with_file(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);

    // In this place, filter instance is a global filter type
    // which is enforced for all layers
    // The trace data will be processed by log_layer independently
    Registry::default().with(filter).with(log_layer).init();

    debug!("Hello, debug!");
    a::b::c::hello();

    error!("Finally, exit");
}
