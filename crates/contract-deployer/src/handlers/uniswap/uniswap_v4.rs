use alloy::{primitives::Address, providers::WalletProvider, sol, sol_types::SolValue};
use anyhow::{Context, Result};
use common::{create2::create2_deploy, helpers::get_salt, state::SharedState};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PoolManager,
    "../artifacts/uniswap-v4/PoolManager.json"
);

pub struct UniswapV4Result {
    pub pool_manager_address: Address,
}

pub async fn deploy_uniswap_v4(state: SharedState) -> Result<UniswapV4Result> {
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

    Ok(UniswapV4Result {
        pool_manager_address,
    })
}
