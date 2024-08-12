mod config;
mod web;

use std::env;

use anyhow::Result;
use config::load_config;
use dotenvy::dotenv;
use tracing::info;
use tracing_panic::panic_hook;
use web::start_web;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();
    std::panic::set_hook(Box::new(panic_hook));

    let config = load_config().await?;

    info!("version {}", env!("CARGO_PKG_VERSION"));
    info!(
        arch = env::consts::ARCH,
        family = env::consts::FAMILY,
        os = env::consts::OS,
    );

    start_web(config).await
}
