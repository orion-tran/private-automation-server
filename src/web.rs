use anyhow::Result;
use axum::{routing::get, Router};
use tracing::info;

use crate::config::Config;

async fn root() -> String {
    "Hello, World!".to_string()
}

pub(crate) async fn start_web(config: Config) -> Result<()> {
    let app = Router::new().route("/", get(root));

    info!(host = config.web.host, "starting server");
    let web_listener = tokio::net::TcpListener::bind(config.web.host).await?;
    axum::serve::serve(web_listener, app).await?;

    Ok(())
}
