
pub fn setup_logging_meter() {
    // #tag::metrics-logging-meter[]
    #[tokio::main]
    async fn main() {
        use std::time::Duration;
        use couchbase::logging_meter::{LoggingMeter, LoggingMeterOptions};
        use tracing_subscriber::layer::SubscriberExt;

        let meter = LoggingMeter::new(Some(
            LoggingMeterOptions::new().emit_interval(Duration::from_secs(300)),
        ));
        let subscriber = tracing_subscriber::registry()
            .with(meter)
            .with(tracing_subscriber::fmt::layer()); // Add any other layers you want, e.g. fmt for logging, ThresholdLoggingTracer, etc.

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global tracing subscriber");
        // ...
    }
    // #end::metrics-logging-meter[]
}

pub fn setup_threshold_logging_tracer() {
    // #tag::tracing-configure[]
    #[tokio::main]
    async fn main() {
        use std::time::Duration;
        use couchbase::threshold_logging_tracer::{ThresholdLoggingTracer, ThresholdLoggingOptions};
        use tracing_subscriber::layer::SubscriberExt;

        let tracer = ThresholdLoggingTracer::new(Some(
            ThresholdLoggingOptions::new()
                .kv_threshold(Duration::from_millis(1000))
                .emit_interval(Duration::from_secs(60))
                .sample_size(5),
        ));
        let subscriber = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(tracer);

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global tracing subscriber");
        // ...
    }
    // #end::tracing-configure[]
}

pub fn otel_metrics() {
    // #tag::otel-metrics[]
    use std::time::Duration;
    use opentelemetry::KeyValue;
    use opentelemetry_sdk::{
        metrics::{SdkMeterProvider, PeriodicReader, Instrument, Stream},
        Resource,
    };
    use opentelemetry_otlp::{MetricExporter, Protocol, WithExportConfig};
    use opentelemetry_otlp::WithTonicConfig;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_opentelemetry::MetricsLayer;
    use tracing::instrument::WithSubscriber;
    use couchbase::cluster::Cluster;
    use couchbase::options::cluster_options::ClusterOptions;
    use couchbase::authenticator::PasswordAuthenticator;

    #[tokio::main]
    async fn main() {
        let meter_provider = setup_otel_meter_provider();

        let subscriber = tracing_subscriber::registry()
            .with(MetricsLayer::new(meter_provider))
            .with(tracing_subscriber::fmt::layer()); // Add any other layers you want

        // Scope the subscriber to your SDK calls.
        // Alternatively, register it globally with:
        // tracing::subscriber::set_global_default(subscriber)
        your_couchbase_sdk_calls()
            .with_subscriber(subscriber)
            .await;
    }

    fn setup_otel_meter_provider() -> SdkMeterProvider {
        // Set up an exporter.
        // This exporter exports traces on the OTLP protocol over GRPC to localhost:4317.
        let exporter = MetricExporter::builder()
            .with_tonic()
            .with_protocol(Protocol::Grpc)
            .with_endpoint("http://localhost:4317")
            .with_compression(opentelemetry_otlp::Compression::Gzip)
            .build()
            .expect("failed to build OTLP metric exporter");

        // Export metrics every second
        let reader = PeriodicReader::builder(exporter)
            .with_interval(Duration::from_secs(1))
            .build();

        let resource = Resource::builder()
            // An OpenTelemetry service name generally reflects the name of your microservice,
            // e.g. "shopping-cart-service".
            .with_attributes([
                KeyValue::new("service.name", "YOUR_SERVICE_NAME_HERE"),
            ])
            .build();

        // Optional workaround for https://github.com/tokio-rs/tracing-opentelemetry/issues/254
        // This ensures the units of the metrics the SDK emits are set correctly to seconds.
        let unit_view = |inst: &Instrument| -> Option<Stream> {
            match inst.name() {
                "db.client.operation.duration" => {
                    Stream::builder()
                        .with_unit("s")
                        .build()
                        .ok()
                }
                _ => None,
            }
        };

        let meter_provider = SdkMeterProvider::builder()
            .with_resource(resource)
            .with_reader(reader)
            .with_view(unit_view)
            .build();

        meter_provider
    }

    async fn your_couchbase_sdk_calls() {
        // Your Couchbase SDK calls here, e.g.:
        let opts = ClusterOptions::new(PasswordAuthenticator::new("username", "password").into());
        let cluster = Cluster::connect("couchbase://localhost", opts).await.unwrap();
        let bucket = cluster.bucket("my_bucket");
        let collection = bucket.default_collection();
        collection.insert("my_key", "my_value", None).await.unwrap();
    }
    // #end::otel-metrics[]
}

pub fn otel_tracer() {
// #tag::otel-tracing[]
use std::time::Duration;
use couchbase::authenticator::PasswordAuthenticator;
use couchbase::cluster::Cluster;
use couchbase::options::cluster_options::ClusterOptions;
use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::{
    trace::{BatchSpanProcessor, Sampler, SdkTracerProvider},
    Resource,
};
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig};
use tracing::instrument::WithSubscriber;
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_opentelemetry::OpenTelemetryLayer;

#[tokio::main]
async fn main() {
    let tracer_provider = setup_otel_tracer_provider();
    let tracer = tracer_provider.tracer("my-app");

    let subscriber = Registry::default()
        .with(OpenTelemetryLayer::new(tracer))
        .with(tracing_subscriber::fmt::layer()); // Add any other layers you want

    // Scope the subscriber to your SDK calls.
    // Alternatively, register it globally with:
    //   tracing::subscriber::set_global_default(subscriber)
    // Note: with a global subscriber, all spans in the process will share the same
    // instrumentation scope name.
    your_couchbase_sdk_calls()
        .with_subscriber(subscriber)
        .await;
}

fn setup_otel_tracer_provider() -> SdkTracerProvider {
    // This exporter exports traces on the OTLP protocol over GRPC to localhost:4317.
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_protocol(Protocol::Grpc)
        .with_endpoint("http://localhost:4317")
        .build()
        .expect("failed to build OTLP exporter");

    let resource = Resource::builder()
        // An OpenTelemetry service name generally reflects the name of your microservice,
        // e.g. "shopping-cart-service".
        .with_attributes([
            KeyValue::new("service.name", "YOUR_SERVICE_NAME_HERE"),
        ])
        .build();

    let tracer_provider = SdkTracerProvider::builder()
        .with_resource(resource)
        // Export every trace: this may be too heavy for production.
        // An alternative is `.with_sampler(Sampler::TraceIdRatioBased(0.01))`
        .with_sampler(Sampler::AlwaysOn)
        // The BatchSpanProcessor will efficiently batch traces and periodically export them.
        .with_span_processor(
            BatchSpanProcessor::builder(exporter)
                .with_batch_config(
                    opentelemetry_sdk::trace::BatchConfigBuilder::default()
                        .with_scheduled_delay(Duration::from_millis(5000))
                        .build(),
                )
                .build(),
        )
        .build();

    tracer_provider
}

async fn your_couchbase_sdk_calls() {
    // Your Couchbase SDK calls here, e.g.:
    let opts = ClusterOptions::new(PasswordAuthenticator::new("username", "password").into());
    let cluster = Cluster::connect("couchbase://localhost", opts).await.unwrap();
    let bucket = cluster.bucket("my_bucket");
    let collection = bucket.default_collection();
    collection.insert("my_key", "my_value", None).await.unwrap();
}
// #end::otel-tracing[]
}
