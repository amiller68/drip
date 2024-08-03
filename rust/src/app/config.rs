use url::Url;

use super::Args;

#[derive(Debug)]
pub struct Config {}

impl From<&Args> for Config {
    fn from(args: &Args) -> Self {
        Self {
            ipfs_rpc_url: match &args.maybe_ipfs_rpc_url {
                Some(url) => url.clone(),
                None => Url::parse("http://localhost:5001").unwrap(),
            },
            eth_rpc_url: match &args.maybe_eth_rpc_url {
                Some(url) => url.clone(),
                None => Url::parse("http://localhost:8545").unwrap(),
            },
            eth_chain_id: args.maybe_eth_chain_id.unwrap_or(31337),
        }
    }
}

impl Config {
    pub fn ipfs_rpc_url(&self) -> &Url {
        &self.ipfs_rpc_url
    }

    pub fn eth_rpc_url(&self) -> &Url {
        &self.eth_rpc_url
    }
    pub fn eth_chain_id(&self) -> u64 {
        self.eth_chain_id
    }
}
