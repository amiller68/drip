use std::convert::TryFrom;

use aes_kw::KekAes256 as Kek;
use secp256k1::ecdh::SharedSecret;

use crate::types::{PrivateKey, PublicKey, Secret, PUBLIC_KEY_SIZE, SECRET_SIZE};

const SHARE_SIZE: usize = 73;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Share(pub(crate) [u8; SHARE_SIZE]);

impl Default for Share {
    fn default() -> Self {
        Share([0; SHARE_SIZE])
    }
}

impl From<[u8; SHARE_SIZE]> for Share {
    fn from(bytes: [u8; SHARE_SIZE]) -> Self {
        Share(bytes)
    }
}

impl From<Share> for [u8; SHARE_SIZE] {
    fn from(share: Share) -> Self {
        share.0
    }
}

impl Share {
    pub fn from_hex(hex: &str) -> Result<Self, ShareError> {
        let mut buff = [0; SHARE_SIZE];
        hex::decode_to_slice(hex, &mut buff).map_err(|_| anyhow::anyhow!("hex decode error"))?;
        Ok(Share::from(buff))
    }

    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn share_secret(secret: Secret, public_key: &PublicKey) -> Result<Self, ShareError> {
        let esk = PrivateKey::generate();
        let epk = esk.public_key();
        let shared_secret = SharedSecret::new(&public_key, &esk.encryption_key());

        let kek = Kek::from(shared_secret.secret_bytes());
        let wrapped = kek
            .wrap_vec(secret.bytes())
            .map_err(|_| anyhow::anyhow!("wrap error"))?;

        let mut share = Share::default();
        share.0[..33].copy_from_slice(&epk.serialize());
        share.0[33..].copy_from_slice(&wrapped);
        Ok(share)
    }

    pub fn recover_secret(&self, private_key: &PrivateKey) -> Result<Secret, ShareError> {
        let epk = PublicKey::try_from(&self.0[..PUBLIC_KEY_SIZE])
            .map_err(|_| anyhow::anyhow!("invalid public key"))?;
        let shared_secret = SharedSecret::new(&epk, &private_key.encryption_key());
        let kek = Kek::from(shared_secret.secret_bytes());
        let unwrapped = kek
            .unwrap_vec(&self.0[PUBLIC_KEY_SIZE..])
            .map_err(|_| anyhow::anyhow!("unwrap error"))?;
        let mut secret = [0; SECRET_SIZE];
        secret.copy_from_slice(&unwrapped);
        Ok(Secret::from(secret))
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

#[derive(Debug, thiserror::Error)]
pub enum ShareError {
    #[error("default: {0}")]
    Default(#[from] anyhow::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_share_secret() {
        let secret = Secret::from_slice(&[0; SECRET_SIZE]);
        let secret_key = PrivateKey::generate();
        let public_key = secret_key.public_key();
        let share = Share::share_secret(secret, &public_key).unwrap();
        let recovered_secret = share.recover_secret(&secret_key).unwrap();
        assert_eq!(secret, recovered_secret);
    }
}
