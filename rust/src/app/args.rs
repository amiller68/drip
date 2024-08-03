use std::error::Error;
use std::fmt::Display;

use alloy::primitives::Address;
use clap::Subcommand;
use url::Url;

use super::ops::Push as PushOp;

pub use clap::Parser;

use std::fmt;

#[async_trait::async_trait]
pub trait Op: Send + Sync {
    type Error: Error + Send + Sync + 'static;
    type Output: Display;

    async fn execute(&self, args: &Args) -> Result<Self::Output, Self::Error>;
}

#[macro_export]
macro_rules! command_enum {
    ($(($variant:ident, $type:ty)),* $(,)?) => {
        #[derive(Subcommand, Debug, Clone)]
        pub enum Command {
            $($variant($type),)*
        }

        #[derive(Debug)]
        pub enum OpOutput {
            $($variant(<$type as Op>::Output),)*
        }

        #[derive(Debug, thiserror::Error)]
        pub enum OpError {
            $(
                #[error(transparent)]
                $variant(<$type as Op>::Error),
            )*
        }

        #[async_trait::async_trait]
        impl Op for Command {
            type Output = OpOutput;
            type Error = OpError;

            async fn execute(&self, args: &Args) -> Result<Self::Output, Self::Error> {
                match self {
                    $(
                        Command::$variant(op) => {
                            op.execute(args).await
                                .map(OpOutput::$variant)
                                .map_err(OpError::$variant)
                        },
                    )*
                }
            }
        }
    };
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    #[clap(long = "ipfs-rpc", short = 'i')]
    pub maybe_ipfs_rpc_url: Option<Url>,
    #[clap(long = "address", short = 'a')]
    pub maybe_eth_address: Option<Address>,
    #[clap(long = "eth-rpc", short = 'e')]
    pub maybe_eth_rpc_url: Option<Url>,
    #[clap(long = "chain-id", short = 'c')]
    pub maybe_eth_chain_id: Option<u64>,
    #[clap(long = "private-key", short = 'p')]
    pub maybe_private_key_hex: Option<String>,
}

use crate::command_enum;

command_enum! {
    (Push, PushOp),
    // Define more commands here
}

impl fmt::Display for OpOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Define more variants here
            OpOutput::Push(output) => write!(f, "{}", output),
        }
    }
}
