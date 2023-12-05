use tracer::OtlpTracerConfig;

pub mod tracer;

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct TracingConfiguration {
    pub otlp: Option<OtlpTracerConfig>,
}
