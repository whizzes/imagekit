use anyhow::Result;
use async_trait::async_trait;
use reqwest::multipart::{Form, Part};
use reqwest::Body;
use std::path::Path;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::ImageKit;

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

pub struct Response {}

#[async_trait]
pub trait Upload {
    async fn upload(&self, opts: Options) -> String;
}

#[async_trait]
impl Upload for ImageKit {
    async fn upload(&self, opts: Options) -> String {
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
        let result = response.text().await.unwrap();

        result
    }
}

#[cfg(test)]
mod tests {
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
        let result = imagekit.upload(opts).await;

        assert_eq!(result, String::default());
    }
}
