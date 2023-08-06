pub mod types;

use std::io::Cursor;


use async_trait::async_trait;
use reqwest::multipart::{Form, Part};
use reqwest::{Body, StatusCode};
use tokio::fs::File;
use tokio::io::BufReader;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::error::{Result, Error};
use crate::ImageKit ;

use self::types::Response;

/// Default Upload Endpoint used by ImageKit
pub const UPLOAD_ENDPOINT: &str = "https://upload.imagekit.io/api/v1/files/upload";

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
    /// Upload Endpoint to use, by default:
    /// `https://upload.imagekit.io/api/v1/files/upload`
    /// is used.
    endpoint: String,
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
            ..Default::default()
        }
    }

    /// Sets the endpoint to use when uploading the file.
    pub fn endpoint<T: AsRef<str> + Into<String>>(mut self, endpoint: T) -> Self {
        self.endpoint = endpoint.into();
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            endpoint: UPLOAD_ENDPOINT.to_string(),
            file: UploadFile::Bytes(vec![]),
            file_name: "untitled".to_string(),
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

        let response = self
            .client
            .post(opts.endpoint)
            .multipart(form)
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
