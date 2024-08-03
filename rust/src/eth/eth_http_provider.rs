use std::ops::Deref;

use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::http::{Client, Http};
use url::Url;

pub struct EthHttpProvider(Box<dyn Provider<Http<Client>>>);

impl EthHttpProvider {
    pub fn new(rpc_url: &Url, chain_id: u64) -> Self {
        let provider = ProviderBuilder::new()
            .with_chain_id(chain_id)
            .on_http(rpc_url.clone());
        Self(Box::new(provider))
    }

    pub fn inner(self) -> Box<dyn Provider<Http<Client>>> {
        return self.0;
    }
}

impl Deref for EthHttpProvider {
    type Target = dyn Provider<Http<Client>>;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
