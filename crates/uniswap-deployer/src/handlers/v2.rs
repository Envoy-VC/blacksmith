use alloy::{providers::ProviderBuilder, signers::local::PrivateKeySigner, sol};
use anyhow::{Context, Result};

use crate::config::Config;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV2Factory,
    "src/artifacts/v2/UniswapV2Factory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    WETH9,
    "src/artifacts/v2/WETH9.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV2RouterV2,
    "src/artifacts/v2/UniswapV2RouterV2.json"
);

pub async fn deploy_uniswap_v2(config: Config) -> Result<()> {
    let signer: PrivateKeySigner = config
        .deployer_private_key()
        .parse()
        .context("Invalid private key")?;
    let deployer_address = signer.address();

    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect_http(config.rpc_url());

    // 1. Deploy WETH9
    let weth = WETH9::deploy(provider.clone())
        .await
        .context("Failed to deploy WETH9")?;
    let weth_address = weth.address();
    println!("WETH9 deployed at: {:?}", weth_address);

    let factory = UniswapV2Factory::deploy(provider.clone(), deployer_address)
        .await
        .context("Failed to deploy UniswapV2Factory")?;
    let factory_address = factory.address();

    println!("UniswapV2Factory deployed at: {:?}", factory_address);

    let router = UniswapV2RouterV2::deploy(provider, *factory_address, *weth_address)
        .await
        .context("Failed to deploy UniswapV2RouterV2")?;
    let router_address = router.address();
    println!("UniswapV2RouterV2 deployed at: {:?}", router_address);
    Ok(())
}
