use std::ops::Deref;

use alloy::contract::ContractInstance;
use alloy::primitives::{Address, Bytes, U256};

use super::EthClient;

pub struct Drop(ContractInstance);

impl Deref for Drop {
    type Target = ContractInstance,
    
}

impl Drop {
    // TODO: hardcode address maybe
    //  and add registry for network
    pub fn from_client(address: Address, client: EthClient) -> Self {
        todo!()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DropError {
    #[error("default: {0}")]
    Default(#[from] anyhow::Error),
}
