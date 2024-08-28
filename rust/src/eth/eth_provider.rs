use alloy::network::Ethereum;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::{
    http::{Client, Http},
    Transport,
};
use url::Url;

pub struct EthProvider<T: Transport>(Box<dyn Provider<T, Ethereum>>);

impl EthProvider<Http<Client>> {
    pub fn http(rpc_url: &Url, chain_id: u64) -> Self {
        let provider = ProviderBuilder::new()
            .with_chain_id(chain_id)
            .on_http(rpc_url.clone());
        Self(Box::new(provider))
    }

    pub fn inner(self) -> dyn Provider<Http<Client>, Ethereum> {
        &*self.0.inner()
    }
}
