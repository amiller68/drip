use crate::types::Share;
use alloy::primitives::FixedBytes;
use alloy::sol_types::sol;

const SHARE_BLOCK_BYTES: usize = 20;

sol! {
    struct ShareSol {
        bytes20[2] value;
    }
}

impl From<Share> for ShareSol {
    fn from(share: Share) -> Self {
        let bytes = share.0;
        let mut value = [FixedBytes::<SHARE_BLOCK_BYTES>::default(); 2];
        value[0].copy_from_slice(&bytes[..20]);
        value[1].copy_from_slice(&bytes[20..]);
        Self { value }
    }
}

impl From<ShareSol> for Share {
    fn from(sol: ShareSol) -> Self {
        let mut all_bytes = Vec::with_capacity(40);
        all_bytes.extend_from_slice(sol.value[0].as_slice());
        all_bytes.extend_from_slice(sol.value[1].as_slice());
        let share = Share::try_from(all_bytes.as_slice()).unwrap();
        share
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_wrapper_rt() {
        let share = Share::default();
        let encoded: ShareSol = share.into();
        let decoded = Share::from(encoded);
        assert_eq!(share, decoded);
    }
}
