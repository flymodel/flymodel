use clap::Parser;
use flymodel_cli::{
    cmds::{Cli, Commands},
    config::ServeConfig,
};
use flymodel_registry::storage::{StorageConfig, StorageProvider};
use flymodel_service::app::start_server;
use futures_util::FutureExt;

use flymodel_migration::Migrator;
use tracing::{debug, Level};

use dotenv::dotenv;

fn get_conf() {}

async fn setup_storage(cli: Cli) -> anyhow::Result<()> {
    let conf = cli.load_config()?;
    let storage = conf.storage.build().await?;
    storage.setup().await?;
    Ok(())
}

async fn serve_server(server: ServeConfig) -> anyhow::Result<()> {
    start_server(
        server.db.to_connection().await?,
        format!("{}:{}", server.bind, server.port),
    )
    .await
}

async fn run(cmd: Cli) -> anyhow::Result<()> {
    match cmd.command {
        Commands::Serve(server) => serve_server(server).await,
        Commands::Migrate(migrate) => migrate.run().await,
        Commands::SetupStorage => setup_storage(cmd).await,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    dotenv().ok();
    Ok(
        match run(Cli::parse())
            .then(|res| async {
                debug!("done");
                res
            })
            .await
        {
            Ok(()) => (),
            Err(e) => tracing::error!("{e}"),
        },
    )
}
