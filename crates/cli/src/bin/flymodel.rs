use clap::Parser;
use flymodel_cli::{
    cmds::{Cli, Commands},
    config::ServeConfig,
};
use flymodel_registry::storage::{StorageConfig, StorageProvider};
use futures_util::FutureExt;

use migration::Migrator;
use tracing::{debug, Level};

use dotenv::dotenv;
#[derive(serde::Deserialize, Debug)]
struct Conf {
    storage: StorageConfig,
}

async fn serve(server: ServeConfig) -> anyhow::Result<()> {
    let bs = std::fs::read("./flymodel.toml")?;
    let utf = String::from_utf8(bs)?;
    let conf: Conf = toml::from_str(&utf)?;

    let storage = conf.storage.build().await?;
    storage.setup().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    dotenv().ok();
    let cmd = Cli::parse();
    let result = (|| async move {
        match cmd.command {
            Commands::Serve(server) => serve(server).await,
            Commands::Migrate(migrate) => migrate.run().await,
        }
    })()
    .then(|res| async {
        debug!("done");
        res
    })
    .await;

    Ok(match result {
        Ok(()) => (),
        Err(e) => tracing::error!("{e}"),
    })
}
