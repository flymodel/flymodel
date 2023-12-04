use clap::Args;
use flymodel::errs::{FlymodelError, FlymodelResult};
use flymodel_migration::hooks::Fixtures;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone, Args)]
pub struct ServeConfig {
    #[arg(short, long, default_value = "9009")]
    pub port: u16,
    #[arg(short, long, default_value = "localhost")]
    pub bind: String,
    #[clap(flatten)]
    pub db: DatabaseConfig,
}

#[derive(Debug, Clone, Args)]
pub struct DatabaseConfig {
    #[arg(short, long, env = "DB_URL")]
    pub database_url: String,
}

#[derive(Debug, Clone, Args)]
pub struct MigrationConfig {
    #[clap(flatten)]
    pub db: DatabaseConfig,
    #[arg(long)]
    pub test_data: Option<Fixtures>,
}

impl DatabaseConfig {
    pub async fn to_connection(&self) -> FlymodelResult<DatabaseConnection> {
        sea_orm::Database::connect(self.database_url.clone())
            .await
            .map_err(FlymodelError::DbConnectionError)
    }
}
