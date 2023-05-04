pub mod client;
pub mod delete;
pub mod management;
pub mod types;
pub mod upload;

pub use client::ImageKit;
pub use delete::Delete;
pub use management::file_details;
pub use types::ErrorResponse;
pub use upload::Upload;

#[cfg(test)]
mod tests {
    use tokio::fs::File;

    use crate::file_details::Details;

    use super::delete::Delete;
    use super::upload::types::FileType;
    use super::upload::{Options, Upload, UploadFile};
    use super::ImageKit;

    #[tokio::test]
    async fn uploads_then_deletes_file() {
        let imagekit = ImageKit::from_env().unwrap();
        let file = File::open("assets/ferris.jpeg").await.unwrap();
        let upload_file = UploadFile::from(file);
        let opts = Options::new(upload_file, "ferris");
        let upload_result = imagekit.upload(opts).await.unwrap();

        assert_eq!(upload_result.file_type, FileType::Image);
        assert_eq!(upload_result.height.unwrap(), 640);
        assert_eq!(upload_result.width.unwrap(), 640);
        
        let detail_result = imagekit.get_file_details(&upload_result.file_id).await;
        assert!(detail_result.is_ok());

        let delete_result = imagekit.delete(upload_result.file_id).await;

        assert!(delete_result.is_ok());
    }
}
