use alloy::{providers::ProviderBuilder, signers::local::PrivateKeySigner};
use anyhow::{Context, Result};

use crate::handlers::uniswap_v2::deploy_uniswap_v2;

use common::{
    config::Config,
    state::{AppState, SharedState},
};

pub mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env();
    let signer: PrivateKeySigner = config
        .deployer_private_key()
        .parse()
        .context("Invalid private key")?;

    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect_http(config.rpc_url());

    let state = AppState {
        config,
        provider: provider.into(),
    };
    let shared_state = SharedState::new(state);

    // Deploy Uniswap V2
    deploy_uniswap_v2(shared_state)
        .await
        .context("Failed to deploy Uniswap V2")?;
    Ok(())
}
