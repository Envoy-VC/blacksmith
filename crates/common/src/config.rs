use std::env;

use alloy::transports::http::reqwest::Url;

pub struct Config {
    rpc_url: Url,
    deployer_private_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
        let deployer_private_key =
            env::var("DEPLOYER_PRIVATE_KEY").expect("DEPLOYER_PRIVATE_KEY must be set");
        Self {
            rpc_url: rpc_url.parse().expect("RPC_URL must be a valid URL"),
            deployer_private_key,
        }
    }

    pub fn rpc_url(&self) -> Url {
        self.rpc_url.clone()
    }

    pub fn deployer_private_key(&self) -> &str {
        &self.deployer_private_key
    }
}
