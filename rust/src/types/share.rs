use std::convert::TryFrom;

const SHARE_SIZE: usize = 40;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Share(pub(crate) [u8; SHARE_SIZE]);

impl Default for Share {
    fn default() -> Self {
        Share([0; SHARE_SIZE])
    }
}

impl From<Share> for [u8; SHARE_SIZE] {
    fn from(share: Share) -> Self {
        share.0
    }
}

impl TryFrom<&[u8]> for Share {
    type Error = ();
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != SHARE_SIZE {
            return Err(());
        }
        let mut share = Share::default();
        share.0.copy_from_slice(bytes);
        Ok(share)
    }
}
