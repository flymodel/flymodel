#[cfg(not(feature = "wasm"))]
pub(crate) fn init_subscriber() {
    use tracing_subscriber::{util::SubscriberInitExt, EnvFilter, FmtSubscriber};
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    subscriber.init();
}

#[cfg(feature = "wasm")]
pub(crate) fn init_subscriber() {
    console_subscriber::init();
}