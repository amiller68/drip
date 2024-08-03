use std::convert::TryFrom;

use alloy::primitives::Address;
use url::Url;

use super::Args;

use drip::{
    eth::{Drop, EthHttpProvider},
    ipfs::{IpfsClient, IpfsRpc},
    types::PrivateKey,
};

pub struct AppState {
    // ipfs_rpc_url
    ipfs_rpc: IpfsRpc<IpfsClient>,

    // Maybe Eth Address
    maybe_eth_address: Option<Address>,

    // Ethereum rpc url
    eth_rpc_url: Url,

    // Ethereum chain id
    eth_chain_id: u64,

    // secret key hex
    maybe_private_key: Option<PrivateKey>,
}

impl TryFrom<&Args> for AppState {
    type Error = AppStateSetupError;

    fn try_from(args: &Args) -> Result<Self, Self::Error> {
        let ipfs_rpc_url = match &args.maybe_ipfs_rpc_url {
            Some(url) => url.clone(),
            None => Url::parse("http://localhost:5001").unwrap(),
        };
        let ipfs_rpc =
            IpfsRpc::<IpfsClient>::try_from(ipfs_rpc_url.clone()).expect("valid ipfs_rpc");
        let eth_rpc_url = match &args.maybe_eth_rpc_url {
            Some(url) => url.clone(),
            None => Url::parse("http://localhost:8545").unwrap(),
        };
        let eth_chain_id = args.maybe_eth_chain_id.unwrap_or(31337);
        let maybe_private_key = match &args.maybe_private_key_hex {
            Some(pkh) => Some(
                PrivateKey::from_hex(pkh)
                    .map_err(|_| AppStateSetupError::InvalidPrivateKey)
                    .unwrap(),
            ),
            None => None,
        };

        Ok(Self {
            ipfs_rpc,
            maybe_eth_address: args.maybe_eth_address,
            eth_rpc_url,
            eth_chain_id,
            maybe_private_key,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateSetupError {
    #[error("invalid private key")]
    InvalidPrivateKey,
}

#[allow(dead_code)]
impl AppState {
    pub fn ipfs_rpc(&self) -> &IpfsRpc<IpfsClient> {
        &self.ipfs_rpc
    }

    pub fn eth_provider(&self) -> EthHttpProvider {
        EthHttpProvider::new(&self.eth_rpc_url, self.eth_chain_id)
    }

    pub fn private_key(&self) -> &PrivateKey {
        &self.maybe_private_key.as_ref().expect("private key")
    }

    pub fn drop(&self) -> Drop<alloy::transports::http::Http<alloy::transports::http::Client>> {
        let address = self.maybe_eth_address.expect("eth address");
        Drop::http(address.clone(), self.eth_provider())
    }
}
