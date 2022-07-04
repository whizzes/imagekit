mod types;

use std::result;

use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::multipart::{Form, Part};
use reqwest::{Body, StatusCode};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::ImageKit;

use self::types::{ErrorResponse, Response};

pub enum UploadFile {
    Binary(File),
}

impl From<File> for UploadFile {
    fn from(file: File) -> Self {
        UploadFile::Binary(file)
    }
}

pub struct Options {
    /// File to upload
    file: UploadFile,
    /// Name to set to the file being uploaded
    ///
    /// The filename must only have alphanumeric characters (a-z, A-Z and/or 0-9),
    /// allowed symbols include `.`, `_`, and `-`.
    file_name: String,
}

#[async_trait]
pub trait Upload {
    /// Uploads an image with the provided `Options`
    async fn upload(&self, opts: Options) -> Result<Response>;
}

#[async_trait]
impl Upload for ImageKit {
    async fn upload(&self, opts: Options) -> Result<Response> {
        let mut form = Form::new();

        form = form.text("fileName", opts.file_name.clone());
        match opts.file {
            UploadFile::Binary(file) => {
                let stream = FramedRead::new(file, BytesCodec::new());
                let file_body = Body::wrap_stream(stream);
                let form_file = Part::stream(file_body)
                    .file_name(opts.file_name)
                    .mime_str("image/jpeg")
                    .unwrap();
                form = form.part("file", form_file);
            }
        }

        let private_key = self.private_key.to_owned();
        let response = self
            .client
            .post(&self.upload_endpoint)
            .basic_auth::<String, String>(private_key, None)
            .multipart(form)
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

#[cfg(test)]
mod tests {
    use super::types::FileType;

    use super::*;

    #[tokio::test]
    async fn upload_image_from_file() {
        let imagekit = ImageKit::from_env().unwrap();
        let file = File::open("assets/ferris.jpeg").await.unwrap();
        let upload_file = UploadFile::from(file);
        let opts = Options {
            file: upload_file,
            file_name: "ferris".to_string(),
        };
        let result = imagekit.upload(opts).await.unwrap();

        assert_eq!(result.file_type, FileType::Image);
        assert_eq!(result.height.unwrap(), 640);
        assert_eq!(result.width.unwrap(), 640);
    }
}
