use std::env;

use anyhow::Result;
use dotenvy::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("version {}", env!("CARGO_PKG_VERSION"));
    info!(
        arch = env::consts::ARCH,
        family = env::consts::FAMILY,
        os = env::consts::OS,
    );

    Ok(())
}
