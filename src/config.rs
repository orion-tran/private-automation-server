use anyhow::Result;
use camino::Utf8PathBuf;
use clap::{command, Parser};
use serde::Deserialize;
use tokio::fs;
use tracing::info;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Config {
  pub(crate) web: WebConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct WebConfig {
  #[serde(default = "default_host")]
  pub(crate) host: String,
}

fn default_host() -> String {
  "0.0.0.0:3000".to_string()
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
  #[clap(short, long, default_value = "config.toml")]
  config: Utf8PathBuf,
}

pub(crate) async fn load_config() -> Result<Config, std::io::Error> {
  let args = Args::parse();

  let config_path = args.config.clone();
  match fs::read_to_string(&config_path).await {
    Ok(config_string) => Ok(toml::from_str(&config_string).expect("could not parse config file")),

    Err(e) => {
      if e.kind() == std::io::ErrorKind::NotFound {
        let config_string = create_default_config(&config_path).await?;
        info!(path = config_path.to_string(), "created default config");

        return Ok(toml::from_str(config_string).expect("could not parse default config file"));
      }

      Err(e)
    }
  }
}

async fn create_default_config(config_path: &Utf8PathBuf) -> Result<&str, std::io::Error> {
  let config_string = include_str!("default_config.toml");
  fs::write(config_path, config_string).await?;
  Ok(config_string)
}
