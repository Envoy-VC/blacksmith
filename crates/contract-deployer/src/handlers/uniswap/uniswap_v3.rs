use std::collections::HashMap;
use std::fs;

use alloy::{hex, primitives::Address, sol, sol_types::SolValue};
use anyhow::{Context, Result};
use common::{
    create2::create2_deploy,
    helpers::{get_salt, link_libraries},
    state::SharedState,
};
use serde::Deserialize;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV3Factory,
    "../artifacts/uniswap-v3/UniswapV3Factory.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    SwapRouter,
    "../artifacts/uniswap-v3/SwapRouter.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    NonfungiblePositionManager,
    "../artifacts/uniswap-v3/NonfungiblePositionManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    NFTDescriptor,
    "../artifacts/uniswap-v3/NFTDescriptor.json"
);

#[derive(Deserialize)]
struct Artifact {
    bytecode: String,
}

pub struct UniswapV3Result {
    pub factory_address: Address,
    pub swap_router_address: Address,
    pub nft_descriptor_address: Address,
    pub position_descriptor_address: Address,
    pub position_manager_address: Address,
    pub weth_address: Address,
}

pub async fn deploy_uniswap_v3(
    state: SharedState,
    weth_address: Address,
) -> Result<UniswapV3Result> {
    // 1. Deploy UniswapV3Factory
    let factory_salt = get_salt("UniswapV3Factory");
    let factory_init_code = UniswapV3Factory::BYTECODE.clone();
    let factory_address = create2_deploy(state.clone(), factory_init_code, factory_salt)
        .await
        .context("Failed to deploy UniswapV3Factory")?;

    // 2. Deploy SwapRouter
    let swap_router_salt = get_salt("SwapRouter");
    let args = (factory_address, weth_address).abi_encode();
    let mut router_init_code = SwapRouter::BYTECODE.clone().to_vec();
    router_init_code.extend(args);

    let swap_router_address =
        create2_deploy(state.clone(), router_init_code.into(), swap_router_salt)
            .await
            .context("Failed to deploy SwapRouter")?;

    // 3. Deploy NFTDescriptorLibrary
    let nft_descriptor_salt = get_salt("NFTDescriptor");
    let nft_descriptor_address = create2_deploy(
        state.clone(),
        NFTDescriptor::BYTECODE.clone(),
        nft_descriptor_salt,
    )
    .await
    .context("Failed to deploy NFTDescriptorLibrary")?;

    // 4. Deploy Position Descriptor
    let position_descriptor_salt = get_salt("NonfungibleTokenPositionDescriptor");
    let mut link_references = HashMap::new();
    let mut contract_refs = HashMap::new();
    contract_refs.insert("NFTDescriptor".to_string(), vec![(1681, 20)]);
    link_references.insert("NFTDescriptor.sol".to_string(), contract_refs);

    let mut libraries = HashMap::new();
    libraries.insert("NFTDescriptor".to_string(), nft_descriptor_address);

    let path = format!(
        "{}/../artifacts/uniswap-v3/NonfungibleTokenPositionDescriptor.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let file = fs::read_to_string(path)?;
    let artifact: Artifact = serde_json::from_str(&file)?;
    let unlinked_bytecode = artifact.bytecode;
    let linked_bytecode = link_libraries(unlinked_bytecode, &link_references, &libraries).unwrap();

    let mut position_descriptor_init_code =
        hex::decode(linked_bytecode).context("Failed to decode bytecode")?;
    let position_descriptor_args = (weth_address, nft_descriptor_address, "ETH").abi_encode();
    position_descriptor_init_code.extend(position_descriptor_args);

    let position_descriptor_address = create2_deploy(
        state.clone(),
        position_descriptor_init_code.into(),
        position_descriptor_salt,
    )
    .await
    .context("Failed to deploy Position Descriptor")?;

    // Deploy NonfungiblePositionManager
    let position_manager_salt = get_salt("NonfungiblePositionManager");
    let position_manager_args =
        (factory_address, weth_address, position_descriptor_address).abi_encode();
    let mut position_manager_init_code = NonfungiblePositionManager::BYTECODE.clone().to_vec();
    position_manager_init_code.extend(position_manager_args);

    let position_manager_address = create2_deploy(
        state.clone(),
        position_manager_init_code.into(),
        position_manager_salt,
    )
    .await
    .context("Failed to deploy Position Manager")?;

    Ok(UniswapV3Result {
        factory_address,
        swap_router_address,
        nft_descriptor_address,
        position_descriptor_address,
        position_manager_address,
        weth_address,
    })
}
