use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct ServeConfig {
    #[arg(short, long, default_value = "9000")]
    pub port: u16,
    #[arg(short, long, default_value = "localhost")]
    pub host: String,
}

#[derive(Debug, Clone, Args)]
pub struct MigrationConfig {
    #[arg(short, long)]
    pub database_url: String,
}
