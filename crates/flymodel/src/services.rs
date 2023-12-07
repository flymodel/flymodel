#[derive(clap::ValueEnum, Clone)]
pub enum Service {
    Graphql,
    Storage,
}
