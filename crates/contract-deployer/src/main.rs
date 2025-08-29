use alloy::{providers::ProviderBuilder, signers::local::PrivateKeySigner};
use anyhow::{Context, Result};

use crate::handlers::uniswap::{deploy_uniswap_v2, deploy_uniswap_v3, deploy_uniswap_v4};

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
    let uniswap_v2 = deploy_uniswap_v2(shared_state.clone())
        .await
        .context("Failed to deploy Uniswap V2")?;

    let uniswap_v3 = deploy_uniswap_v3(shared_state.clone(), uniswap_v2.weth_address)
        .await
        .context("Failed to deploy Uniswap V3")?;

    let uniswap_v4 = deploy_uniswap_v4(shared_state.clone(), uniswap_v2.weth_address)
        .await
        .context("Failed to deploy Uniswap V4")?;

    println!("Uniswap V2 Factory: {:?}", uniswap_v2.factory_address);
    println!("Uniswap V2 Router: {:?}", uniswap_v2.router_address);
    println!("Uniswap V2 WETH: {:?}\n", uniswap_v2.weth_address);

    println!("Uniswap V3 Factory: {:?}", uniswap_v3.factory_address);
    println!(
        "Uniswap V3 Swap Router: {:?}",
        uniswap_v3.swap_router_address
    );
    println!(
        "Uniswap V3 NFT Descriptor: {:?}",
        uniswap_v3.nft_descriptor_address
    );
    println!(
        "Uniswap V3 Position Descriptor: {:?}",
        uniswap_v3.position_descriptor_address
    );
    println!(
        "Uniswap V3 Position Manager: {:?}\n",
        uniswap_v3.position_manager_address
    );

    println!("Uniswap V4 Permit2: {:?}", uniswap_v4.permit2_address);
    println!(
        "Uniswap V4 Pool Manager: {:?}",
        uniswap_v4.pool_manager_address
    );
    println!(
        "Uniswap V4 Position Descriptor: {:?}",
        uniswap_v4.position_descriptor_address
    );
    println!(
        "Uniswap V4 Position Manager: {:?}",
        uniswap_v4.position_manager_address
    );
    println!("Uniswap V4 Quoter: {:?}", uniswap_v4.quoter_address);
    println!("Uniswap V4 State View: {:?}", uniswap_v4.state_view_address);

    Ok(())
}
