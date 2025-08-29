use alloy::{primitives::Address, providers::WalletProvider, sol, sol_types::SolValue};
use anyhow::{Context, Result};
use common::{create2::create2_deploy, helpers::get_salt, state::SharedState};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PoolManager,
    "../artifacts/uniswap-v4/PoolManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PositionDescriptor,
    "../artifacts/uniswap-v4/PositionDescriptor.json"
);

pub struct UniswapV4Result {
    pub pool_manager_address: Address,
    pub position_descriptor_address: Address,
}

pub async fn deploy_uniswap_v4(
    state: SharedState,
    weth_address: Address,
) -> Result<UniswapV4Result> {
    let provider = &state.provider;
    let deployer_address = provider.default_signer_address();

    // 1. Deploy PoolManager
    let pool_manager_salt = get_salt("PoolManager");
    let mut pool_manager_init_code = PoolManager::BYTECODE.clone().to_vec();
    pool_manager_init_code.extend(deployer_address.abi_encode());
    let pool_manager_address = create2_deploy(
        state.clone(),
        pool_manager_init_code.into(),
        pool_manager_salt,
    )
    .await
    .context("Failed to deploy PoolManager")?;

    // 2. Deploy PositionDescriptor
    let position_descriptor_salt = get_salt("PositionDescriptor");
    let mut position_descriptor_init_code = PositionDescriptor::BYTECODE.clone().to_vec();
    let position_descriptor_args = (pool_manager_address, weth_address, "ETH").abi_encode();
    position_descriptor_init_code.extend(position_descriptor_args);
    let position_descriptor_address = create2_deploy(
        state.clone(),
        position_descriptor_init_code.into(),
        position_descriptor_salt,
    )
    .await
    .context("Failed to deploy PositionDescriptor")?;

    Ok(UniswapV4Result {
        pool_manager_address,
        position_descriptor_address,
    })
}
