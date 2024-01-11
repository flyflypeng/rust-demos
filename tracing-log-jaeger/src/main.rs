use opentelemetry::trace::TraceError;
use opentelemetry_sdk::trace::Tracer;
use tracing::{debug, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

mod a {
    pub mod b {
        pub mod c {
            use tracing::{info, instrument};
            #[instrument(skip_all)]
            pub fn hello(v: &str) {
                info!("Hello, {}!", v);
            }
        }
    }
}

// add jaeger tracer
fn init_tracer() -> Result<Tracer, TraceError> {
    opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("tracing-log-jaeger")
        .install_simple()
}

fn main() {
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    let log_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_target(true)
        .with_level(true)
        .with_file(true);

    let tracer = init_tracer().expect("Failed to init tracer");

    // In this place, filter instance is a global filter type
    // which is enforced for all layers
    // The trace data will be processed by log_layer independently
    Registry::default()
        .with(filter)
        .with(log_layer)
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    debug!("Hello, debug!");
    a::b::c::hello("World");

    error!("Exit, error!");

    opentelemetry::global::shutdown_tracer_provider();
}
