use std::path::PathBuf;

use async_trait::async_trait;

use drip::types::Cid;

use crate::app::{AppState, Op};

#[derive(Debug, clap::Args, Clone)]
pub struct Push {
    #[clap(short, long)]
    input: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum PushError {
    #[error("default error: {0}")]
    Default(#[from] anyhow::Error),
}

#[async_trait]
impl Op for Push {
    type Error = PushError;
    type Output = Cid;

    async fn execute(&self, args: &crate::app::Args) -> Result<Self::Output, Self::Error> {
        Ok(Cid::default())
    }
}
