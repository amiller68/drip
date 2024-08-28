use std::convert::TryFrom;

use alloy::network::EthereumWallet;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::http::{Client, Http};
use alloy_chains::NamedChain;
use url::Url;

use super::Args;

use drip::{
    ipfs::{IpfsClient, IpfsRpc},
    types::PrivateKey,
};

pub struct AppState {
    // ipfs_rpc_url
    ipfs_rpc: IpfsRpc<IpfsClient>,

    // Ethereum rpc url
    eth_rpc_url: Url,

    // Ethereum chain id
    eth_chain: NamedChain,

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
        let eth_chain = NamedChain::try_from(eth_chain_id).expect("valid chain id");
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
            eth_rpc_url,
            eth_chain,
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

    pub fn eth_provider(&self) -> impl Provider<Http<Client>> {
        let private_key = self
            .maybe_private_key
            .as_ref()
            .expect("private key")
            .signer()
            .0;
        ProviderBuilder::new()
            .with_recommended_fillers()
            .with_chain(self.eth_chain)
            // .wallet(EthereumWallet::from(private_key))
            .on_http(self.eth_rpc_url.clone())
    }

    pub fn private_key(&self) -> &PrivateKey {
        &self.maybe_private_key.as_ref().expect("private key")
    }
}
