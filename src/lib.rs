pub mod client;
pub mod delete;
pub mod management;
pub mod types;
pub mod upload;
pub mod url;

pub use client::ImageKit;
pub use delete::Delete;
pub use management::file_details;
pub use types::ErrorResponse;
pub use upload::Upload;
pub use url::Transformation;
pub use url::TransformationPosition;
pub use url::Url;

#[cfg(test)]
mod tests {
    use tokio::fs::File;

    use super::delete::Delete;
    use super::file_details::Details;
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

    #[tokio::test]
    async fn uploads_and_retrieve_information() {
        let imagekit = ImageKit::from_env().unwrap();
        let file = File::open("assets/ferris.jpeg").await.unwrap();
        let upload_file = UploadFile::from(file);
        let opts = Options::new(upload_file, "ferris");
        let upload_result = imagekit.upload(opts).await.unwrap();
        let detail_result = imagekit.get_file_details(upload_result.file_id).await;
        assert!(detail_result.is_ok());
    }
}

#[cfg(test)]
mod url_tests {
    use super::url::Options;
    use super::ImageKit;
    use super::Transformation;
    use super::Url;

    #[tokio::test]
    async fn url_transformation_for_path() {
        let imagekit = ImageKit::from_env().unwrap();
        let transformation = Transformation::new()
            .width(200)
            .height(200)
            .aspect_raio("1-1");
        let options = Options::new(transformation).path("ferris_cTgKr8mAO");
        let result = imagekit.url(options);
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn url_transformation_for_aspect_ratio() {
        let imagekit = ImageKit::from_env().unwrap();
        let transformation = Transformation::new().aspect_raio("2-1").height(200);
        let options = Options::new(transformation).path("ferris_cTgKr8mAO");
        let result = imagekit.url(options);
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn url_transformation_for_query() {
        let imagekit = ImageKit::from_env().unwrap();
        let transformation = Transformation::new().width(200);
        let options = Options::new(transformation)
            .path("ferris_cTgKr8mAO")
            .transformation_position(crate::TransformationPosition::Query);
        let result = imagekit.url(options);
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn url_transformation_for_src() {
        let imagekit = ImageKit::from_env().unwrap();
        let transformation = Transformation::new().width(200);
        let src = format!("{}/ferris", imagekit.url_endpoint);
        let options = Options::new(transformation).src(src);
        let result = imagekit.url(options);
        dbg!(&result);
        assert!(result.is_ok());
    }
}
