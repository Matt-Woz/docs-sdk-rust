use log::LevelFilter;
use std::io::Write;
use tracing_subscriber::Layer;

pub fn tracing_subscriber_logging() {
    // tag::tracing-logging[]
    #[tokio::main]
    async fn main() {
        use tracing_subscriber::{layer::SubscriberExt, EnvFilter};
        use couchbase::logging_meter::LoggingMeter;
        use couchbase::threshold_logging_tracer::ThresholdLoggingTracer;

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| {
                EnvFilter::new("info")
            });

        let subscriber = tracing_subscriber::registry()
            .with(ThresholdLoggingTracer::new(None)) // Enables periodic slow operation logging
            .with(LoggingMeter::new(None)) // Enables periodic operation metrics logging
            .with(tracing_subscriber::fmt::layer().with_filter(filter));

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global tracing subscriber");
    }
    // end::tracing-logging[]
}

pub fn env_logger() {
    // tag::env-logger[]
    env_logger::init();
    // end::env-logger[]

    // tag::env-logger-extended[]
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("rustls"), LevelFilter::Warn)
        .init();
    // end::env-logger-extended[]
}
