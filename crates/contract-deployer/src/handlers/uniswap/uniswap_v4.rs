use alloy::{primitives::Address, providers::WalletProvider, sol, sol_types::SolValue};
use anyhow::{Context, Result};
use common::{create2::create2_deploy, helpers::get_salt, state::SharedState};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Permit2,
    "../artifacts/uniswap-v4/Permit2.json"
);

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

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PositionManager,
    "../artifacts/uniswap-v4/PositionManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Quoter,
    "../artifacts/uniswap-v4/Quoter.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    StateView,
    "../artifacts/uniswap-v4/StateView.json"
);

pub struct UniswapV4Result {
    pub permit2_address: Address,
    pub pool_manager_address: Address,
    pub position_descriptor_address: Address,
    pub position_manager_address: Address,
    pub quoter_address: Address,
    pub state_view_address: Address,
}

pub async fn deploy_uniswap_v4(
    state: SharedState,
    weth_address: Address,
) -> Result<UniswapV4Result> {
    let provider = &state.provider;
    let deployer_address = provider.default_signer_address();

    // Set Up Helpers Permit2
    let permit2_salt = get_salt("Permit2");
    let permit2_init_code = Permit2::BYTECODE.clone();
    let permit2_address = create2_deploy(state.clone(), permit2_init_code, permit2_salt)
        .await
        .context("Failed to deploy Permit2")?;

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

    // 3. Deploy PositionManager
    // PositionManager::deploy(provider, poolManager, permit2, unsubscribeGasLimit, tokenDescriptor, weth9)
    let position_manager_salt = get_salt("PositionManager");
    let position_manager_args = (
        pool_manager_address,
        permit2_address,
        100_000,
        position_descriptor_address,
        weth_address,
    )
        .abi_encode();
    let mut position_manager_init_code = PositionManager::BYTECODE.clone().to_vec();
    position_manager_init_code.extend(position_manager_args);
    let position_manager_address = create2_deploy(
        state.clone(),
        position_manager_init_code.into(),
        position_manager_salt,
    )
    .await
    .context("Failed to deploy PositionManager")?;

    // 4. Deploy Quoter
    let quoter_salt = get_salt("Quoter");
    let quoter_args = (pool_manager_address).abi_encode();
    let mut quoter_init_code = Quoter::BYTECODE.clone().to_vec();
    quoter_init_code.extend(quoter_args);
    let quoter_address = create2_deploy(state.clone(), quoter_init_code.into(), quoter_salt)
        .await
        .context("Failed to deploy Quoter")?;

    // 5. Deploy StateView
    let state_view_salt = get_salt("StateView");
    let state_view_args = (pool_manager_address).abi_encode();
    let mut state_view_init_code = StateView::BYTECODE.clone().to_vec();
    state_view_init_code.extend(state_view_args);
    let state_view_address =
        create2_deploy(state.clone(), state_view_init_code.into(), state_view_salt)
            .await
            .context("Failed to deploy StateView")?;

    Ok(UniswapV4Result {
        permit2_address,
        pool_manager_address,
        position_descriptor_address,
        position_manager_address,
        quoter_address,
        state_view_address,
    })
}
