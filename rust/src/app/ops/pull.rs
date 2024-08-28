use std::convert::TryFrom;
use std::fs::File;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;

use alloy::primitives::Address;
use async_trait::async_trait;

use drip::eth::{CidSol, DropContract, ShareSol};
use drip::types::{Cid, MhCode, Secret, Share};

use crate::app::{AppState, Op};

#[derive(Debug, clap::Args, Clone)]
pub struct Pull {
    #[clap(short, long)]
    output: PathBuf,
    #[clap(short, long)]
    address: Address,
}

#[derive(Debug, thiserror::Error)]
pub enum PullError {
    #[error("default error: {0}")]
    Default(#[from] anyhow::Error),
}

#[async_trait]
impl Op for Pull {
    type Error = PullError;
    type Output = String;

    async fn execute(&self, state: &AppState) -> Result<Self::Output, Self::Error> {
        let ipfs_rpc = state.ipfs_rpc();
        let eth_provider = state.eth_provider();
        let private_key = state.private_key();

        let output = self.output.clone();
        let address = self.address;

        let drop_contract = DropContract::new(address, eth_provider);

        let cid_sol_value = drop_contract.cid().call().await.expect("valid root call");
        let cid_sol = CidSol {
            value: cid_sol_value._0,
        };
        let cid = Cid::try_from(cid_sol).expect("valid cid token");
        println!("cid: {:?}", cid);

        let share_sol_value = drop_contract
            .claim()
            .call()
            .await
            .expect("valid share call");
        let share_sol = ShareSol {
            value: share_sol_value._0,
        };
        let share = Share::from(share_sol);
        println!("share: {:?}", share);

        let enc_data = ipfs_rpc
            .cat_data_send_safe(&cid)
            .await
            .expect("valid ipfs cat");

        let secret = share
            .recover_secret(&private_key)
            .expect("valid secret recovery");
        let dec_data = secret.decrypt(&enc_data).expect("valid decryption");

        let mut file = File::create(output).expect("valid file creation");
        file.write_all(&dec_data).expect("valid file write");

        Ok("done".to_string())
    }
}
