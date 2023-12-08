use flymodel_members::server::{MembershipConfig, Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Server::new(MembershipConfig::default()).serve().await
}
