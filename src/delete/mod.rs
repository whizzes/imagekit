use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::{StatusCode, Url};

use crate::client::FILES_ENDPOINT;
use crate::{ErrorResponse, ImageKit};

#[async_trait]
pub trait Delete {
    /// Deletes the file with the provided File ID
    async fn delete<T: ToString + Send>(&self, file_id: T) -> Result<()>;
}

#[async_trait]
impl Delete for ImageKit {
    async fn delete<T: ToString + Send>(&self, file_id: T) -> Result<()> {
        let url_string = format!("{}/{}", FILES_ENDPOINT, file_id.to_string());
        let endpoint_url = Url::parse(&url_string).unwrap();
        let private_key = self.private_key.to_owned();
        let response = self
            .client
            .delete(endpoint_url)
            .basic_auth::<String, String>(private_key, None)
            .send()
            .await
            .unwrap();

        if matches!(response.status(), StatusCode::NO_CONTENT) {
            return Ok(());
        }

        let result = response.json::<ErrorResponse>().await.unwrap();

        bail!(result.message);
    }
}
