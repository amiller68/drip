use std::convert::TryFrom;
use std::io::Read;
use std::ops::Deref;

use alloy::signers::local::PrivateKeySigner as PPrivateKeySigner;
use rand::rngs::OsRng;
use secp256k1::{generate_keypair as _generate_keypair, Secp256k1};
use secp256k1::{PublicKey as SPublicKey, SecretKey as SSecretKey}; // Assuming AES-256, so 32 bytes key

pub const SECRET_SIZE: usize = 32;
pub const PUBLIC_KEY_SIZE: usize = 33;

fn generate_keypair() -> (SSecretKey, SPublicKey) {
    let mut rng = OsRng::default();
    _generate_keypair(&mut rng)
}

pub struct Secret([u8; SECRET_SIZE]);

impl Default for Secret {
    fn default() -> Self {
        Secret([0; SECRET_SIZE])
    }
}

impl Deref for Secret {
    type Target = [u8; SECRET_SIZE];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; SECRET_SIZE]> for Secret {
    fn from(bytes: [u8; SECRET_SIZE]) -> Self {
        Secret(bytes)
    }
}

impl Into<PPrivateKeySigner> for Secret {
    fn into(self) -> PPrivateKeySigner {
        let bytes = self.0;
        PPrivateKeySigner::from_slice(&bytes).unwrap()
    }
}

impl Secret {
    pub fn new() -> Self {
        let (secret_key, _) = generate_keypair();
        Secret(secret_key.secret_bytes())
    }

    pub fn bytes(&self) -> &[u8] {
        self.0.as_ref()
    }
}

pub struct PublicKey(SPublicKey);

impl Deref for PublicKey {
    type Target = SPublicKey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; PUBLIC_KEY_SIZE]> for PublicKey {
    fn from(bytes: [u8; PUBLIC_KEY_SIZE]) -> Self {
        PublicKey(SPublicKey::from_slice(&bytes).unwrap())
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = PublicKeyError;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != PUBLIC_KEY_SIZE {
            return Err(anyhow::anyhow!("invalid public key size").into());
        }
        let mut buff = [0; PUBLIC_KEY_SIZE];
        buff.copy_from_slice(bytes);
        Ok(buff.into())
    }
}

impl PublicKey {
    pub fn from_hex(hex: &str) -> Result<Self, PublicKeyError> {
        let mut buff = [0; PUBLIC_KEY_SIZE];
        hex::decode_to_slice(hex, &mut buff).map_err(|_| anyhow::anyhow!("hex decode error"))?;
        Ok(buff.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PublicKeyError {
    #[error("default: {0}")]
    Default(#[from] anyhow::Error),
}

pub struct PrivateKeySigner(PPrivateKeySigner);

impl Deref for PrivateKeySigner {
    type Target = PPrivateKeySigner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct PrivateEncryptionKey(SSecretKey);

impl Deref for PrivateEncryptionKey {
    type Target = SSecretKey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct PrivateKey(Secret);

impl From<[u8; SECRET_SIZE]> for PrivateKey {
    fn from(secret: [u8; SECRET_SIZE]) -> Self {
        Self(Secret::from(secret))
    }
}

impl PrivateKey {
    pub fn from_hex(hex: &str) -> Result<Self, PrivateKeyError> {
        let mut buff = [0; SECRET_SIZE];
        hex::decode_to_slice(hex, &mut buff).map_err(|_| anyhow::anyhow!("hex decode error"))?;
        Ok(Self::from(buff))
    }

    pub fn generate() -> Self {
        let (secret_key, _) = generate_keypair();
        Self(Secret(secret_key.secret_bytes()))
    }

    pub fn public_key(&self) -> PublicKey {
        let secp = Secp256k1::new();
        let secret_key = SSecretKey::from_slice(&self.0 .0).unwrap();
        let public_key = secret_key.public_key(&secp);
        PublicKey(public_key)
    }

    pub fn encryption_key(&self) -> PrivateEncryptionKey {
        let secret_key = SSecretKey::from_slice(&self.0 .0).unwrap();
        PrivateEncryptionKey(secret_key)
    }

    pub fn signer(&self) -> PrivateKeySigner {
        let private_key_signer = PPrivateKeySigner::from_slice(&self.0 .0).unwrap();
        PrivateKeySigner(private_key_signer)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PrivateKeyError {
    #[error("default: {0}")]
    Default(#[from] anyhow::Error),
}
