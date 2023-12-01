use crate::config::{MigrationConfig, ServeConfig};
use clap::{Parser, Subcommand};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Serve(ServeConfig),
    #[command(subcommand)]
    Migrate(Migration),
}

#[derive(Debug, Clone, Subcommand)]
pub enum Migration {
    Up(MigrationConfig),
    Down(MigrationConfig),
}

impl Migration {
    pub async fn run(self) -> anyhow::Result<()> {
        let conn = sea_orm::Database::connect(match &self {
            Self::Down(conf) => conf.database_url.clone(),
            Self::Up(conf) => conf.database_url.clone(),
        })
        .await?;
        match self {
            Self::Up(config) => Migrator::up(&conn, None).await,
            Self::Down(config) => Migrator::down(&conn, None).await,
        }?;
        Ok(())
    }
}
