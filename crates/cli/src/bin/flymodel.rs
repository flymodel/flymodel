use std::process::ExitCode;

use clap::Parser;
use flymodel_cli::{
    cmds::{Cli, Commands},
    config::ServeConfig,
};
use flymodel_service::app::start_server;
use futures_util::FutureExt;

use tracing::{debug, level_filters::LevelFilter, Level};

use dotenv::dotenv;
use tracing_subscriber::{
    filter,
    reload::{self, Handle, Layer},
};

async fn setup_storage<S>(
    cli: Cli,
    reload_handle: reload::Handle<filter::LevelFilter, S>,
) -> anyhow::Result<()> {
    let conf = cli.load_config()?;
    conf.log.reload(reload_handle)?;
    let storage = conf.storage.build().await?;
    storage.setup().await?;
    Ok(())
}

async fn serve_server<S>(
    cli: Cli,
    server: &ServeConfig,
    reload_handle: reload::Handle<filter::LevelFilter, S>,
) -> anyhow::Result<()> {
    let conf = cli.load_config()?;
    conf.log.reload(reload_handle)?;

    start_server(
        server.db.to_connection().await?,
        format!("{}:{}", server.bind, server.port),
        conf.tracer(),
    )
    .await
}

async fn run<S>(
    cmd: Cli,
    reload_handle: reload::Handle<filter::LevelFilter, S>,
) -> anyhow::Result<()> {
    match cmd.command {
        Commands::Serve(ref server) => serve_server(cmd.clone(), server, reload_handle).await,
        Commands::Migrate(migrate) => migrate.run().await,
        Commands::SetupStorage => setup_storage(cmd, reload_handle).await,
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let filter = filter::LevelFilter::WARN;
    let (_filter, reload_handle): (
        Layer<LevelFilter, tracing_subscriber::fmt::Subscriber>,
        Handle<LevelFilter, tracing_subscriber::fmt::Subscriber>,
    ) = reload::Layer::new(filter);

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    dotenv().ok();
    match run(Cli::parse(), reload_handle)
        .then(|res| async {
            debug!("done");
            res
        })
        .await
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            tracing::error!("{e}");
            ExitCode::FAILURE
        }
    }
}
