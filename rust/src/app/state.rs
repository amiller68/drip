use super::config::Config;
use drip::{
    eth::EthClient,
    ipfs::{IpfsClient, IpfsRpc},
};

#[derive(Clone)]
pub struct AppState {
    ipfs_rpc: IpfsRpc<IpfsClient>,
    eth_rpc: EthClient,
}

impl From<&Config> for AppState {
    fn from(config: &Config) -> Self {
        let ipfs_rpc = IpfsRpc::<IpfsClient>::try_from(config.ipfs_rpc_url().clone()).unwrap();
        let eth_rpc = EthClient::new(config.eth_rpc_url().clone(), config.eth_chain_id()).unwrap();
        Self { ipfs_rpc, eth_rpc }
    }
}

#[allow(dead_code)]
impl AppState {
    pub fn ipfs_rpc(&self) -> &IpfsRpc<IpfsClient> {
        &self.ipfs_rpc
    }

    pub fn eth_rpc(&self) -> &EthClient {
        &self.eth_rpc
    }
}
