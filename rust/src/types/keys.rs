use std::convert::TryFrom;
use std::ops::Deref;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use rand_core::{OsRng, RngCore};

use alloy::signers::{local::PrivateKeySigner as PPrivateKeySigner, Signer};
use secp256k1::{generate_keypair as _generate_keypair, Secp256k1};
use secp256k1::{PublicKey as SPublicKey, SecretKey as SSecretKey}; // Assuming AES-256, so 32 bytes key

pub const NONCE_SIZE: usize = 16;
pub const SECRET_SIZE: usize = 32;
pub const PRIVATE_KEY_SIZE: usize = 32;
pub const PUBLIC_KEY_SIZE: usize = 33;

fn generate_keypair() -> (SSecretKey, SPublicKey) {
    let mut rng = OsRng::default();
    _generate_keypair(&mut rng)
}

#[derive(Debug, PartialEq)]
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
        let mut buff = [0; SECRET_SIZE];
        OsRng.fill_bytes(&mut buff);
        Self(buff)
    }

    pub fn from_slice(data: &[u8]) -> Result<Self, SecretError> {
        if data.len() != SECRET_SIZE {
            return Err(anyhow::anyhow!("invalid secret size").into());
        }
        let mut buff = [0; SECRET_SIZE];
        buff.copy_from_slice(data);
        Ok(buff.into())
    }

    pub fn bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, SecretError> {
        let mut rng = OsRng::default();
        let key: &Key<Aes256Gcm> = self.bytes().into();

        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut rng); // 96-bits; unique per message
        let ciphertext = cipher
            .encrypt(&nonce, data.as_ref())
            .map_err(|_| anyhow::anyhow!("encrypt error"))?;

        let mut out = Vec::with_capacity(NONCE_SIZE + ciphertext.len());

        out.extend_from_slice(nonce.as_ref());
        out.extend_from_slice(ciphertext.as_ref());

        Ok(out)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, SecretError> {
        let key: &Key<Aes256Gcm> = self.bytes().into();
        let nonce = Nonce::from_slice(&data[..NONCE_SIZE]);
        let cipher = Aes256Gcm::new(&key);
        let decrypted = cipher
            .decrypt(nonce, &data[NONCE_SIZE..])
            .map_err(|_| anyhow::anyhow!("decrypt error"))?;

        Ok(decrypted.to_vec())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SecretError {
    #[error("default: {0}")]
    Default(#[from] anyhow::Error),
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

pub struct PrivateKeySigner(pub PPrivateKeySigner);

impl Deref for PrivateKeySigner {
    type Target = PPrivateKeySigner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct PrivateKey(SSecretKey);

impl From<[u8; PRIVATE_KEY_SIZE]> for PrivateKey {
    fn from(secret: [u8; PRIVATE_KEY_SIZE]) -> Self {
        Self(SSecretKey::from_slice(&secret).unwrap())
    }
}

impl Deref for PrivateKey {
    type Target = SSecretKey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PrivateKey {
    pub fn from_hex(hex: &str) -> Result<Self, PrivateKeyError> {
        // Optionally strip off 0x if it begins with one
        let hex = hex.strip_prefix("0x").unwrap_or(hex);
        let mut buff = [0; PRIVATE_KEY_SIZE];
        hex::decode_to_slice(hex, &mut buff).map_err(|_| anyhow::anyhow!("hex decode error"))?;
        Ok(Self::from(buff))
    }

    pub fn generate() -> Self {
        let (secret_key, _) = generate_keypair();
        Self(secret_key)
    }

    pub fn public_key(&self) -> PublicKey {
        let secp = Secp256k1::new();
        let public_key = self.0.public_key(&secp);
        PublicKey(public_key)
    }
    pub fn signer(&self) -> PrivateKeySigner {
        let private_key_signer = PPrivateKeySigner::from_slice(&self.secret_bytes()).unwrap();
        PrivateKeySigner(private_key_signer.with_chain_id(Some(31337)))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PrivateKeyError {
    #[error("default: {0}")]
    Default(#[from] anyhow::Error),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_private_key_from_hex() {
        let hex_secret = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let private_key = PrivateKey::from_hex(hex_secret).unwrap();
        assert_eq!(hex_secret, hex::encode(private_key.secret_bytes()));
    }
}
