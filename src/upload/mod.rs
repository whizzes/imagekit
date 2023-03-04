pub mod types;

use std::io::Cursor;

use anyhow::{bail, Result};
use async_trait::async_trait;
use reqwest::multipart::{Form, Part};
use reqwest::{Body, StatusCode};
use tokio::fs::File;
use tokio::io::BufReader;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{ErrorResponse, ImageKit};

use self::types::Response;

pub enum UploadFile {
    Binary(File),
    Bytes(Vec<u8>),
}

impl From<File> for UploadFile {
    fn from(file: File) -> Self {
        UploadFile::Binary(file)
    }
}

impl From<Vec<u8>> for UploadFile {
    fn from(value: Vec<u8>) -> Self {
        UploadFile::Bytes(value)
    }
}

/// Options sent to the server as part of the `Form` when uploding a file.
///
/// Refer: https://docs.imagekit.io/api-reference/upload-file-api/server-side-file-upload#request-structure-multipart-form-data
pub struct Options {
    /// File to upload
    file: UploadFile,
    /// Name to set to the file being uploaded
    ///
    /// The filename must only have alphanumeric characters (a-z, A-Z and/or 0-9),
    /// allowed symbols include `.`, `_`, and `-`.
    file_name: String,
}

impl Options {
    /// Creates a new instance of `Options` with the provided `UploadFile` and
    /// file name.
    pub fn new<T: ToString>(file: UploadFile, file_name: T) -> Self {
        Self {
            file,
            file_name: file_name.to_string(),
        }
    }
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
            UploadFile::Bytes(file_bytes) => {
                let cursor = Cursor::new(file_bytes);
                let buf_reader = BufReader::new(cursor);
                let stream = FramedRead::new(buf_reader, BytesCodec::new());
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
