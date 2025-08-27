use anyhow::{Context, Result};

use crate::{config::Config, handlers::v2::deploy_uniswap_v2};

pub mod config;
pub mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();

    // Deploy Uniswap V2
    deploy_uniswap_v2(config)
        .await
        .context("Failed to deploy Uniswap V2")?;
    Ok(())
}
