#![allow(dead_code)]
//#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod error;
pub mod eth;
pub mod ipfs;
pub mod types;
pub mod version;

#[cfg(target_arch = "wasm32")]
mod wasm;

/// Prelude for the saturn library exporting the most commonly used types and traits.
///
/// ```rust
/// use drip::prelude::*;
/// ```
pub mod prelude {
    pub use crate::error::{BlossomError, BlossomResult};
    pub use crate::eth::{CidSol, DropContract, ShareSol};
    pub use crate::ipfs::IpfsRpc;
    pub use crate::types;
}
