use flymodel::tls::TlsConf;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{ExportConfig, Protocol, WithExportConfig};
use opentelemetry_sdk::{
    metrics::{
        reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
        MeterProvider,
    },
    trace::{self, BatchConfig, Sampler},
    Resource,
};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Clone, serde::Deserialize, Debug)]
pub struct OtlpTracerConfig {
    pub target: String,
    pub tls: Option<TlsConf>,
}

#[derive(Clone)]
pub struct OtlpTracer {
    pub tracer: opentelemetry_sdk::trace::Tracer,
    pub meter: MeterProvider,
}

impl OtlpTracerConfig {
    fn export_config(&self) -> ExportConfig {
        let endpoint = {
            if self.tls.is_some() { "https" } else { "http" }.to_string() + "://" + &self.target
        };
        ExportConfig {
            endpoint: endpoint,
            timeout: TIMEOUT,
            protocol: Protocol::Grpc,
        }
    }

    pub fn new_tracer_provider<S>(&self, service_name: S) -> anyhow::Result<OtlpTracer>
    where
        S: ToString,
    {
        let attrs = vec![
            opentelemetry_semantic_conventions::resource::SERVICE_NAME
                .string(service_name.to_string()),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
            KeyValue::new(
                "service.environment",
                std::env::var("ENVIRONMENT").unwrap_or("dev".to_string()),
            ),
            KeyValue::new("vcs.commit", env!("GIT_HASH")),
            KeyValue::new("vcs.branch", env!("GIT_BRANCH")),
            KeyValue::new("vcs.status", env!("GIT_STATUS")),
        ];

        let resource = Resource::new(attrs.clone());
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::AlwaysOn)
                    .with_resource(resource.clone()),
            )
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_export_config(self.export_config()),
            )
            .with_batch_config(
                BatchConfig::default()
                    .with_max_queue_size(4096)
                    .with_max_export_timeout(TIMEOUT)
                    .with_max_export_batch_size(1028)
                    .with_scheduled_delay(Duration::from_millis(2500)),
            )
            .install_batch(opentelemetry_sdk::runtime::Tokio)?;

        let meter = opentelemetry_otlp::new_pipeline()
            .metrics(opentelemetry_sdk::runtime::Tokio)
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_export_config(self.export_config()),
            )
            .with_resource(resource)
            .with_period(Duration::from_millis(2500))
            .with_timeout(TIMEOUT)
            .with_aggregation_selector(DefaultAggregationSelector::new())
            .with_temporality_selector(DefaultTemporalitySelector::new())
            .build()?;
        Ok(OtlpTracer { tracer, meter })
    }
}
