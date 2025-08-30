use alloy::{
    hex,
    network::TransactionBuilder,
    primitives::{address, keccak256, Address, Bytes, FixedBytes},
    providers::Provider,
    rpc::types::TransactionRequest,
    sol_types::SolValue,
};
use anyhow::{Context, Result};

use crate::state::SharedState;

fn compute_create2_address(factory: Address, init_code: Bytes, salt: FixedBytes<32>) -> Address {
    // Create2 Address = keccak256(0xff + factory + salt + bytecode_hash)
    let hash = keccak256(init_code.as_ref());
    let bytes = (Bytes::from([0xff]), factory, salt, hash).abi_encode_packed();
    let addr_hash = keccak256(&bytes);
    Address::from_slice(&addr_hash[12..])
}

pub async fn create2_deploy(
    state: SharedState,
    init_code: Bytes,
    salt: FixedBytes<32>,
) -> Result<Address> {
    let provider = &state.provider;
    let deterministic_deployer = address!("0x4e59b44847b379578588920cA78FbF26c0B4956C");
    let computed_address = compute_create2_address(deterministic_deployer, init_code.clone(), salt);

    // Maintain Idempotency
    let existing_code = provider
        .get_code_at(computed_address)
        .await
        .context(format!("Failed to fetch code at {}", computed_address))?;
    if !existing_code.is_empty() {
        return Ok(computed_address);
    }

    let calldata = [salt.as_slice(), init_code.as_ref()].concat();

    let request = TransactionRequest::default()
        .with_to(deterministic_deployer)
        .with_input(calldata)
        .with_gas_limit(15_000_000);

    let deploy_tx = provider
        .send_transaction(request)
        .await
        .context("Failed to deploy contract")?;
    let tx_hash = deploy_tx.tx_hash().clone();
    let receipt = deploy_tx.get_receipt().await.context(format!(
        "Failed to get receipt for contract deployment 0x{}",
        hex::encode(tx_hash)
    ))?;

    if receipt.status() == false {
        return Err(anyhow::anyhow!("Failed to deploy contract"));
    }

    let code = provider
        .get_code_at(computed_address)
        .await
        .context(format!("Failed to fetch code at {}", computed_address))?;

    if code.is_empty() {
        return Err(anyhow::anyhow!("Empty code found at {}", computed_address));
    }

    Ok(computed_address)
}
