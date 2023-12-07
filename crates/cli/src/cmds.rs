use crate::{
    config::{MigrationConfig, ServeConfig},
    log::LoggingConfig,
};
use clap::{Parser, Subcommand};
use config::Config;
use flymodel::config::auth::{AuthConfiguration, AuthHandlers};
use flymodel_migration::Migrator;
use flymodel_registry::storage::StorageConfig;
use flymodel_tracing::{tracer::OtlpTracerConfig, TracingConfiguration};
use sea_orm_migration::MigratorTrait;
use std::path::PathBuf;
use tracing::warn;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true, default_value = "./flymodel.toml")]
    pub config: PathBuf,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Serve(ServeConfig),
    #[command(subcommand)]
    Migrate(Migration),
    SetupStorage,

    Upsert,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Migration {
    Up(MigrationConfig),
    Down(MigrationConfig),
}

impl Migration {
    pub async fn run(self) -> anyhow::Result<()> {
        let conf = match &self {
            Self::Down(c) => c,
            Self::Up(c) => c,
        };

        let conn = conf.db.to_connection().await?;
        let conf = match &self {
            Self::Up(c) => c,
            Self::Down(c) => c,
        };
        Migrator::init(conf.test_data.clone());
        Ok(match self {
            Self::Up(..) => Migrator::up(&conn, conf.steps).await,
            Self::Down(..) => Migrator::down(&conn, conf.steps).await,
        }?)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Conf {
    pub storage: StorageConfig,
    pub tracing: Option<TracingConfiguration>,
    #[serde(default)]
    pub log: LoggingConfig,
    #[serde(default)]
    pub auth: AuthConfiguration,
}

impl Conf {
    pub fn tracer(&self) -> Option<OtlpTracerConfig> {
        match &self.tracing {
            Some(TracingConfiguration { otlp, .. }) => otlp.clone(),
            None => None,
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.auth.handler == AuthHandlers::NoOp {
            warn!("no authorizer is configured, using noop authorizer")
        }
        Ok(())
    }
}

impl Cli {
    pub fn load_config(&self) -> anyhow::Result<Conf> {
        let conf = Config::builder()
            .add_source(config::File::from(self.config.clone()))
            .add_source(config::Environment::with_prefix("fm").separator("_"))
            .build()?
            .try_deserialize::<Conf>()?;
        conf.validate()?;
        Ok(conf)
    }
}

mod test {
    #[test]
    fn test_server_load_toml() -> anyhow::Result<()> {
        let cli = super::Cli {
            command: super::Commands::SetupStorage,
            config: "../../conf/flymodel.toml".into(),
        };
        let _conf = cli.load_config()?;
        Ok(())
    }

    #[test]
    fn test_server_load_yaml() -> anyhow::Result<()> {
        let cli = super::Cli {
            command: super::Commands::SetupStorage,
            config: "../../conf/flymodel.yaml".into(),
        };
        let _ = cli.load_config()?;
        Ok(())
    }
}
