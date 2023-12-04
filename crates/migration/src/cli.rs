use sea_orm_migration::prelude::*;

pub async fn run_main() {
    cli::run_cli(crate::Migrator).await
}
