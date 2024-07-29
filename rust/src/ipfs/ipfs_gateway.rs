use std::path::PathBuf;

use crate::types::Cid;
use http::uri::Scheme;
use reqwest::Client;
use url::Url;

/// A wrapper around a gateway url
pub struct IpfsGateway(Url);

impl Default for IpfsGateway {
    fn default() -> Self {
        Self(Url::parse("http://127.0.0.1:8080").unwrap())
    }
}

impl From<Url> for IpfsGateway {
    fn from(url: Url) -> Self {
        Self(url)
    }
}

impl IpfsGateway {
    #[allow(dead_code)]
    pub fn new(url: Url) -> Self {
        Self(url)
    }

    // TODO: this isn't working quite right
    pub async fn get(&self, cid: &Cid, path: Option<PathBuf>) -> Result<Vec<u8>, IpfsGatewayError> {
        let maybe_port = self.0.port();
        let scheme = Scheme::try_from(self.0.scheme())?;
        let host_str = match maybe_port {
            Some(port) => format!("{}:{}", self.0.host_str().unwrap(), port),
            None => self.0.host_str().unwrap().to_string(),
        };
        let url = match path {
            Some(p) => Url::parse(&format!(
                "{}://{}/ipfs/{}/{}",
                scheme,
                host_str,
                cid,
                p.display()
            )),
            None => Url::parse(&format!("{}://{}/ipfs/{}", scheme, host_str, cid)),
        }?;
        // TODO: not 100% sure why I need to use trust_dns here, but this works
        #[cfg(not(target_arch = "wasm32"))]
        let client = Client::builder().hickory_dns(true).build()?;
        #[cfg(target_arch = "wasm32")]
        let client = Client::builder().build()?;
        let resp = client.get(url).send().await?;
        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IpfsGatewayError {
    #[error("default: {0}")]
    Default(#[from] anyhow::Error),
    #[error("url: {0}")]
    Url(#[from] url::ParseError),
    #[error("scheme: {0}")]
    Scheme(#[from] http::uri::InvalidUri),
    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}
