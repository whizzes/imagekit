use std::fmt::Display;

use anyhow::Result;
use async_trait::async_trait;
use reqwest::StatusCode;

use crate::{client::FILES_ENDPOINT, upload::types::Response, ImageKit, error::Error};

#[async_trait]
pub trait Details {
    /// Given a file id retrieves the information within
    async fn get_file_details<T: AsRef<str> + Display + Send>(&self, id: T) -> Result<Response, Error>;
}

#[async_trait]
impl Details for ImageKit {
    async fn get_file_details<T: AsRef<str> + Display + Send>(&self, id: T) -> Result<Response, Error> {
        let response = self
            .client
            .get(format!("{FILES_ENDPOINT}/{id}/details"))
            .send()
            .await?;

        if matches!(response.status(), StatusCode::OK) {
            let result = response.json::<Response>().await.unwrap();

            return Ok(result);
        }

        let error = Error::from_error_code(response.status(), &response.text().await?);
        return Err(error);
    }
}
