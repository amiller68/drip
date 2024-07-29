use std::sync::Arc;

use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::Signer;
use alloy::transports::http::{Client, Http};
use url::Url;

pub struct EthClient {
    provider: Box<dyn Provider<Http<Client>>>,
    signer: Option<Arc<dyn Signer>>,
}

impl EthClient {
    pub fn http(rpc_url: &Url, chain_id: u64) -> Result<Self, EthClientError> {
        let provider = ProviderBuilder::new()
            .with_chain_id(chain_id)
            .on_http(rpc_url.clone());
        Ok(Self {
            provider: Box::new(provider),
            signer: None,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EthClientError {
    #[error("default: {0}")]
    Default(#[from] anyhow::Error),
}
