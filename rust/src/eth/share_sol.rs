use crate::types::Share;
use alloy::primitives::FixedBytes;
use alloy::sol_types::sol;

const SHARE_BLOCK_BYTES: usize = 20;

sol! {
    struct ShareSol {
        bytes20[4] value;
    }
}

impl From<Share> for ShareSol {
    fn from(share: Share) -> Self {
        let bytes = share.0;
        let mut value = [FixedBytes::<SHARE_BLOCK_BYTES>::default(); 4];
        value[0].copy_from_slice(&bytes[..20]);
        value[1].copy_from_slice(&bytes[20..40]);
        value[2].copy_from_slice(&bytes[40..60]);
        let mut fill = Vec::with_capacity(20);
        fill.extend_from_slice(&bytes[60..73]);
        fill.extend_from_slice(&[0u8; 20 - 13]);
        value[3].copy_from_slice(&fill);
        Self { value }
    }
}

impl From<ShareSol> for Share {
    fn from(sol: ShareSol) -> Self {
        let mut all_bytes = Vec::with_capacity(SHARE_BLOCK_BYTES * 4);
        all_bytes.extend_from_slice(sol.value[0].as_slice());
        all_bytes.extend_from_slice(sol.value[1].as_slice());
        all_bytes.extend_from_slice(sol.value[2].as_slice());
        all_bytes.extend_from_slice(sol.value[3].as_slice());

        let bytes = all_bytes.as_slice()[..73].to_vec();

        let share = Share::try_from(bytes.as_slice()).unwrap();
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
