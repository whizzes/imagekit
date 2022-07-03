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

        let response = self
            .client
            .post(&self.upload_endpoint)
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
    async fn test_post_form_file() {
        let url = "http://httpbin.org/post?a=1&b=true";
        let get_json = reqwest_multipart_form(url).await.unwrap();

        println!("users: {:#?}", get_json);
    }
}
