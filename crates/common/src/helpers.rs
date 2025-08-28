use rand::RngCore;

use alloy::{
    hex,
    primitives::{keccak256, Address, FixedBytes},
};

pub fn link_libraries(
    mut bytecode: String,
    link_references: &std::collections::HashMap<
        String,
        std::collections::HashMap<String, Vec<(usize, usize)>>,
    >,
    libraries: &std::collections::HashMap<String, Address>,
) -> Result<String, String> {
    // Strip `0x` if present
    if bytecode.starts_with("0x") {
        bytecode = bytecode[2..].to_string();
    }

    for (file_name, contracts) in link_references {
        for (contract_name, refs) in contracts {
            let address = libraries
                .get(contract_name)
                .ok_or_else(|| format!("Missing link library name {}", contract_name))?;

            // Lowercase hex, no `0x`, 40 chars
            let addr_hex = hex::encode(address.as_slice());

            for (byte_start, byte_length) in refs {
                let start = byte_start * 2; // hex chars
                let length = byte_length * 2;

                if start + length > bytecode.len() {
                    return Err(format!(
                        "Link reference out of range: {}:{}",
                        file_name, contract_name
                    ));
                }

                bytecode.replace_range(start..start + length, &addr_hex);
            }
        }
    }

    Ok(format!("0x{}", bytecode))
}

pub fn get_salt(contract_name: &str) -> FixedBytes<32> {
    keccak256(contract_name.as_bytes())
}

pub fn generate_random_salt() -> FixedBytes<32> {
    let mut rng = rand::rng();
    let mut salt_bytes = [0u8; 32];
    rng.fill_bytes(&mut salt_bytes);
    FixedBytes::new(salt_bytes)
}
