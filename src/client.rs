use anyhow::{bail, Result};
use reqwest::Client;
use std::env::var;

pub const UPLOAD_ENDPOINT: &'static str = "https://upload.imagekit.io/api/v1/files/upload";

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
    pub(crate) upload_endpoint: String,
    pub(crate) public_key: String,
    pub(crate) private_key: String,
    pub(crate) url_endpoint: String,
    pub(crate) client: Client,
}

impl ImageKit {
    pub fn new<T: ToString>(public_key: T, private_key: T, url_endpoint: T) -> Self {
        let client = Client::new();

        Self {
            upload_endpoint: UPLOAD_ENDPOINT.to_string(),
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

    /// Returns a mutable reference to the `upload_endpoint` used by this
    /// ImageKit client instance. Can be used to update the instance value
    /// or retrieve the value.
    ///
    /// ```
    /// use imagekit::client::ImageKit;
    ///
    /// let mut image_kit = ImageKit::new(
    ///    "your_public_api_key",
    ///    "your_private_api_key",
    ///    "https://ik.imagekit.io/your_imagekit_id/",
    /// );
    /// let new_endpoint = String::from("https://upload.example.com/api/v1/files/upload");
    ///
    /// *image_kit.upload_endpoint() = new_endpoint.clone();
    ///
    /// assert_eq!(image_kit.upload_endpoint().to_owned(), new_endpoint);
    /// ```
    pub fn upload_endpoint(&mut self) -> &mut String {
        &mut self.upload_endpoint
    }
}

#[cfg(test)]
mod tests {
    use super::ImageKit;

    #[test]
    fn updates_the_upload_endpoint() {
        let mut image_kit = ImageKit::new(
            "your_public_api_key",
            "your_private_api_key",
            "https://ik.imagekit.io/your_imagekit_id/",
        );
        let new_endpoint = String::from("https://upload.example.com/api/v1/files/upload");

        *image_kit.upload_endpoint() = new_endpoint.clone();

        assert_eq!(image_kit.upload_endpoint, new_endpoint);
    }
}
