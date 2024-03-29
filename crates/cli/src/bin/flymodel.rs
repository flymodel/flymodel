use std::{process::ExitCode, sync::Arc};

use clap::Parser;
use flymodel_cli::{
    cmds::{Cli, Commands},
    config::ServeConfig,
};
use flymodel_service::app::start_server;
use futures_util::FutureExt;

use tracing::{level_filters::LevelFilter, Level};

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
    let tracer = conf.tracer();
    let storage = conf.storage.build().await?;
    start_server(
        server.db.to_connection().await?,
        format!("{}:{}", server.bind, server.port),
        conf.server.temp_dir,
        tracer,
        Arc::new(storage),
        conf.server.tls,
        cli.dry,
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
        Commands::Upsert => unimplemented!(),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let filter = filter::LevelFilter::WARN;
    let (filter, reload_handle): (
        Layer<LevelFilter, tracing_subscriber::fmt::Subscriber>,
        Handle<LevelFilter, tracing_subscriber::fmt::Subscriber>,
    ) = reload::Layer::new(filter);

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    dotenv().ok();

    run(Cli::parse(), reload_handle)
        .then(|res| async {
            let exit = match res {
                Ok(()) => {
                    tracing::info!(name: "exit", "exit success");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    tracing::error!(name: "exit", "exit error: {e}");
                    ExitCode::FAILURE
                }
            };
            drop(filter);
            exit
        })
        .await
}
