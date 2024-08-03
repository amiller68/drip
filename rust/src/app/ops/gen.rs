use std::convert::TryFrom;
use std::path::PathBuf;

use async_trait::async_trait;

use drip::types::{Cid, Secret, Share};

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
    type Output = (Cid, Share);

    async fn execute(&self, args: &crate::app::Args) -> Result<Self::Output, Self::Error> {
        let state = AppState::try_from(args).unwrap();
        let ipfs_rpc = state.ipfs_rpc();
        let private_key = state.private_key();

        Ok(Cid::default())
    }
}
