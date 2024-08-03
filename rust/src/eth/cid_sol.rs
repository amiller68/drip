use crate::types::Cid;
use alloy::primitives::FixedBytes;
use alloy::sol_types::sol;

const CID_BLOCK_BYTES_SIZE: usize = 32;

sol! {
    struct CidSol {
        bytes32[2] value;
    }
}

impl From<Cid> for CidSol {
    fn from(cid: Cid) -> Self {
        let buff_0 = [0u8; 32];
        let buff_1 = [0u8; 32];
        let bytes = cid.to_bytes();
        let mut value = [FixedBytes::<CID_BLOCK_BYTES_SIZE>::default(); 2];
        let all_bytes = bytes
            .iter()
            .chain(buff_0.iter())
            .chain(buff_1.iter())
            .take(64)
            .copied()
            .collect::<Vec<u8>>();
        let (bytes_0, bytes_1) = all_bytes.split_at(32);
        value[0].copy_from_slice(&bytes_0);
        value[1].copy_from_slice(&bytes_1);
        Self { value }
    }
}

impl TryFrom<CidSol> for Cid {
    type Error = String;

    fn try_from(sol: CidSol) -> Result<Self, Self::Error> {
        let mut all_bytes = Vec::with_capacity(64);
        all_bytes.extend_from_slice(sol.value[0].as_slice());
        all_bytes.extend_from_slice(sol.value[1].as_slice());
        let cid = Cid::try_from(all_bytes.as_slice())
            .map_err(|_| "Invalid CID -- no parse".to_string())?;
        Ok(cid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cid_wrapper_rt() {
        let cid = Cid::default();
        let cid_sol = CidSol::from(cid);
        let decoded = Cid::try_from(cid_sol).unwrap();
        assert_eq!(cid, decoded);
    }
}
