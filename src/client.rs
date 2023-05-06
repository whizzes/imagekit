use std::env::var;

use anyhow::{bail, Result};
use http_auth_basic::Credentials;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, ClientBuilder};

pub const FILES_ENDPOINT: &str = "https://api.imagekit.io/v1/files";

/// An ImageKit.io API Client Instance
///
/// Requires a PublicKey, PrivateKey and URL Endpoint from your ImageKit
/// account.
///
/// By default the `upload_endpoint` value used is the `UPLOAD_ENDPOINT`
/// constant:
///
/// ```ignore
/// https://upload.imagekit.io/api/v1/files/upload
/// ```
///
/// If you want to set a custom upload endpoint, you can use the
/// `upload_endpoint` method.
pub struct ImageKit {
    #[allow(dead_code)]
    pub(crate) public_key: String,
    pub(crate) private_key: String,
    #[allow(dead_code)]
    pub(crate) url_endpoint: String,
    pub(crate) client: Client,
}

impl ImageKit {
    pub fn new<T: ToString>(public_key: T, private_key: T, url_endpoint: T) -> Self {
        let creds = Credentials::new(&private_key.to_string(), "").as_http_header();
        let mut headers = HeaderMap::new();

        headers.insert(AUTHORIZATION, HeaderValue::from_str(&creds).unwrap());

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            public_key: public_key.to_string(),
            private_key: private_key.to_string(),
            url_endpoint: url_endpoint.to_string(),
            client,
        }
    }

    pub fn from_env() -> Result<Self> {
        let public_key = ImageKit::env("IMAGEKIT_PUBLIC_KEY")?;
        let private_key = ImageKit::env("IMAGEKIT_PRIVATE_KEY")?;
        let url_endpoint = ImageKit::env("IMAGEKIT_URL_ENDPOINT")?;
        let imagekit = Self::new(public_key, private_key, url_endpoint);

        Ok(imagekit)
    }

    fn env(key: &str) -> Result<String> {
        match var(key) {
            Ok(value) => Ok(value),
            Err(err) => bail!(err),
        }
    }
}
