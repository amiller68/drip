use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::path::PathBuf;

use async_trait::async_trait;

use drip::eth::{CidSol, DropContract, ShareSol};
use drip::types::{MhCode, Secret, Share};

use crate::app::{AppState, Op};

#[derive(Debug, clap::Args, Clone)]
pub struct Gen {
    #[clap(short, long)]
    input: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum GenError {
    #[error("default error: {0}")]
    Default(#[from] anyhow::Error),
}

#[async_trait]
impl Op for Gen {
    type Error = GenError;
    type Output = String;

    async fn execute(&self, state: &AppState) -> Result<Self::Output, Self::Error> {
        let ipfs_rpc = state.ipfs_rpc();
        let eth_provider = state.eth_provider();

        let private_key = state.private_key();
        let mut file = File::open(&self.input).expect("valid file path");
        let mut data: Vec<u8> = vec![];
        file.read_to_end(&mut data).expect("valid input file");

        let secret = Secret::new();
        let share = Share::new(&secret, &private_key.public_key()).expect("valid share");

        let enc_data = secret.encrypt(&data).expect("valid encryption");

        let enc_data_cursor = Cursor::new(enc_data);
        let cid = ipfs_rpc
            .add_data_send_safe(MhCode::Blake3_256, enc_data_cursor)
            .await
            .expect("valid ipfs push");

        let share_sol = ShareSol::from(share);
        let cid_sol = CidSol::from(cid);

        let drop_contract = DropContract::deploy(&eth_provider, cid_sol.value, share_sol.value)
            .await
            .map_err(|e| anyhow::anyhow!("failed to deploy drop contract: {}", e))?;

        let address = drop_contract.address();

        let value = serde_json::json!({
            "address": address,
            "cidHex": hex::encode(cid.to_bytes()),
            "shareHex": share.to_hex(),
        });

        drop_contract
            .shareWith(share_sol.value, private_key.signer().address())
            .call()
            .await
            .expect("valid shareWith");

        Ok(value.to_string())
    }
}
