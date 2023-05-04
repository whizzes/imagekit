use std::fmt::Display;

use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::StatusCode;

use crate::{client::FILES_ENDPOINT, upload::types::Response, ErrorResponse, ImageKit};

#[async_trait]
pub trait Details {
    /// Given a file id retrieves the information within
    async fn get_file_details<T: Display + Send>(&self, id: T) -> Result<Response>;
}

#[async_trait]
impl Details for ImageKit {
    async fn get_file_details<T: Display + Send>(&self, id: T) -> Result<Response> {
        let response = self
            .client
            .get(format!("{FILES_ENDPOINT}/{id}/details"))
            .basic_auth::<&str, &str>(&self.private_key, None)
            .send()
            .await
            .unwrap();

        if matches!(response.status(), StatusCode::OK) {
            let result = response.json::<Response>().await.unwrap();

            return Ok(result);
        }

        let result = response.json::<ErrorResponse>().await.unwrap();

        bail!(result.message);
    }
}
