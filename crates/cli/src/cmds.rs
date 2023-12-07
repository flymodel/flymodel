use crate::{
    config::{MigrationConfig, ServeConfig},
    log::LoggingConfig,
};
use anyhow::Context;
use clap::{Parser, Subcommand};
use flymodel_migration::Migrator;
use flymodel_registry::storage::StorageConfig;

use flymodel_tracing::{tracer::OtlpTracerConfig, TracingConfiguration};
use sea_orm_migration::MigratorTrait;
use std::path::PathBuf;

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
}

impl Conf {
    pub fn tracer(&self) -> Option<OtlpTracerConfig> {
        match &self.tracing {
            Some(TracingConfiguration { otlp, .. }) => otlp.clone(),
            None => None,
        }
    }
}

impl Cli {
    pub fn load_config(&self) -> anyhow::Result<Conf> {
        let ext = match self.config.extension() {
            Some(ext) => ext,
            None => anyhow::bail!("no extension detected"),
        }
        .to_str()
        .context("extension is unknown to os")?;
        let config = std::fs::read_to_string(&self.config)?;
        Ok(match ext {
            "toml" => toml::from_str(&config)?,
            "yaml" => serde_yaml::from_str(&config)?,
            ext => anyhow::bail!("invalid format: {}", ext),
        })
    }
}

mod test {

    #[test]
    fn test_server_load_conf() -> anyhow::Result<()> {
        let cli = super::Cli {
            command: super::Commands::SetupStorage,
            config: "../../conf/flymodel.toml".into(),
        };
        let _conf = cli.load_config()?;

        // assert_eq!(
        //     conf.tracing,
        //     TracingConfiguration {
        //         otlp: Some(OtlpTracerConfig {
        //             target: "localhost:4317".into(),
        //             ..Default::default()
        //         })
        //     }
        // );

        let cli = super::Cli {
            command: super::Commands::SetupStorage,
            config: "../../conf/flymodel.yaml".into(),
        };
        let _ = cli.load_config()?;
        Ok(())
    }
}
