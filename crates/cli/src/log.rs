use tracing::level_filters::LevelFilter;
use tracing_subscriber::reload;

#[derive(Debug, Clone, serde::Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub struct LoggingConfig {
    pub level: LogLevel,
}

impl LoggingConfig {
    pub fn reload<S>(&self, reload_handle: reload::Handle<LevelFilter, S>) -> anyhow::Result<()> {
        reload_handle.modify(|filter| {
            *filter = match self.level {
                LogLevel::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
                LogLevel::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
                LogLevel::Info => tracing_subscriber::filter::LevelFilter::INFO,
                LogLevel::Warn => tracing_subscriber::filter::LevelFilter::WARN,
                LogLevel::Error => tracing_subscriber::filter::LevelFilter::ERROR,
            }
        })?;
        Ok(())
    }
}
