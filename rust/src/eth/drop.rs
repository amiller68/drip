use alloy::contract::{ContractInstance, Interface};
use alloy::json_abi::JsonAbi;
use alloy::network::Ethereum;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::http::{Client, Http};

use super::EthHttpProvider;

pub struct Drop<P>(ContractInstance<P, Box<dyn Provider<P>>, Ethereum>);

impl Drop<Http<Client>> {
    // TODO: hardcode address maybe
    //  and add registry for network
    pub fn http(address: Address, provider: EthHttpProvider) -> Self {
        // TODO: make this update based on abi maybe
        let abi = JsonAbi::parse([
            "constructor(bytes32[2] memory _cid)",
            "function cid() public view returns (bytes32[2] memory)",
            "function claim() public view returns (bytes20[2] memory)",
            "function shareWith(bytes20[2] memory _share, address recipient) public",
        ])
        .unwrap();
        Self(ContractInstance::new(
            address,
            provider.inner(),
            Interface::new(abi),
        ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DropError {
    #[error("default: {0}")]
    Default(#[from] anyhow::Error),
}
