use crate::config::{DatabaseConfig, MigrationConfig, ServeConfig};
use clap::{Parser, Subcommand};
use flymodel_migration::Migrator;
use flymodel_registry::storage::StorageConfig;
use sea_orm::DatabaseConnection;
use sea_orm_migration::{migrator, MigratorTrait};
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
            Self::Up(..) => Migrator::up(&conn, None).await,
            Self::Down(..) => Migrator::down(&conn, None).await,
        }?)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Conf {
    pub storage: StorageConfig,
}

impl Cli {
    pub fn load_config(&self) -> anyhow::Result<Conf> {
        let ext = match self.config.extension() {
            Some(ext) => ext,
            None => anyhow::bail!("no extension detected"),
        }
        .to_str();
        if let Some(ext) = ext {
            let config = std::fs::read_to_string(&self.config)?;
            let conf: Conf = match ext {
                "toml" => toml::from_str(&config)?,
                "yaml" => serde_yaml::from_str(&config)?,
                ext => anyhow::bail!("invalid format: {}", ext),
            };
            Ok(conf)
        } else {
            anyhow::bail!("no extension detected")
        }
    }
}

mod test {

    #[test]
    fn test_server_load_conf() -> anyhow::Result<()> {
        let cli = super::Cli {
            command: super::Commands::SetupStorage,
            config: "../../flymodel.toml".into(),
        };
        let _ = cli.load_config()?;
        let cli = super::Cli {
            command: super::Commands::SetupStorage,
            config: "../../flymodel.yaml".into(),
        };
        let _ = cli.load_config()?;
        Ok(())
    }
}
