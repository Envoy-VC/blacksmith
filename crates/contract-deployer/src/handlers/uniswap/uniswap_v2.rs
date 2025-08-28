use alloy::{primitives::Address, providers::WalletProvider, sol, sol_types::SolValue};
use anyhow::{Context, Result};
use common::{create2::create2_deploy, helpers::get_salt, state::SharedState};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV2Factory,
    "../artifacts/uniswap-v2/UniswapV2Factory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    WETH9,
    "../artifacts/uniswap-v2/WETH9.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV2RouterV2,
    "../artifacts/uniswap-v2/UniswapV2RouterV2.json"
);

pub struct UniswapV2Result {
    pub factory_address: Address,
    pub weth_address: Address,
    pub router_address: Address,
}

pub async fn deploy_uniswap_v2(state: SharedState) -> Result<UniswapV2Result> {
    let provider = &state.provider;
    let deployer_address = provider.default_signer_address();
    println!("Deployer address: {:?}", deployer_address);

    // 1. Deploy WETH9
    let weth_init_code = WETH9::BYTECODE.clone();
    let weth_address = create2_deploy(state.clone(), weth_init_code, get_salt("WETH9"))
        .await
        .context("Failed to deploy WETH9")?;

    let factory_args = deployer_address;
    let encoded_factory_args = factory_args.abi_encode();
    let mut factory_init_code = UniswapV2Factory::BYTECODE.to_vec();
    factory_init_code.extend(encoded_factory_args);

    let factory_address = create2_deploy(
        state.clone(),
        factory_init_code.into(),
        get_salt("UniswapV2Factory"),
    )
    .await
    .context("Failed to deploy UniswapV2Factory")?;

    let router_args = (factory_address, weth_address);
    let encoded_router_args = router_args.abi_encode();
    let mut router_init_code = UniswapV2RouterV2::BYTECODE.to_vec();
    router_init_code.extend(encoded_router_args);

    let router_address = create2_deploy(
        state.clone(),
        router_init_code.into(),
        get_salt("UniswapV2RouterV2"),
    )
    .await
    .context("Failed to deploy UniswapV2RouterV2")?;

    Ok(UniswapV2Result {
        factory_address,
        weth_address,
        router_address,
    })
}
