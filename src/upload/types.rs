use serde::Deserialize;

/// An object containing the file or file version's id (versionId) and name.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub id: String,
    pub name: String,
}

/// Array of AITags associated with the image. If no AITags are set, it
/// will be null. These tags can be added using the google-auto-tagging
/// or aws-auto-tagging extensions.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiTag {
    pub name: String,
    pub confidence: f32,
    pub source: String,
}

/// The type of file could be either `image` or `non-image`.
#[derive(Debug, Deserialize, PartialEq)]
pub enum FileType {
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "non-image")]
    NonImage,
}

/// Response struct returned from successful requests to the ImageKit API.
///
/// Refer: https://docs.imagekit.io/api-reference/upload-file-api/server-side-file-upload#response-code-and-structure-json
/// Fields Documentation: https://docs.imagekit.io/api-reference/upload-file-api/server-side-file-upload#understanding-response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// Unique fileId. Store this fileld in your database, as this will be used
    /// to perform update action on this file
    pub file_id: String,
    /// Name of the file or folder.
    pub name: String,
    /// Size of the image file in Bytes
    pub size: u64,
    /// An object containing the file or file version's id (versionId) and name.
    pub version_info: VersionInfo,
    /// The relative path of the file. In the case of an image, you can use
    /// this path to construct different transformations.
    pub file_path: String,
    /// A publicly accessible URL of the file.
    pub url: String,
    /// The type of file could be either `image` or `non-image`.
    pub file_type: FileType,
    /// Height of the image in pixels (Only for images)
    pub height: Option<u64>,
    /// Width of the image in pixels (Only for Images)
    pub width: Option<u64>,
    /// In the case of an image, a small thumbnail URL.
    /// Since the API has not normalized some fields such as thumbnail
    /// A provided workaround is to attempt the parsing from "thumbnail_url" or "thumbnail"
    /// CHECK: Could this incur in repeating the same proccess twice so it may have a performance impact?
    #[serde(alias = "thumbnail")]
    pub thumbnail_url: Option<String>,
    /// Array of AITags associated with the image. If no AITags are set, it
    /// will be null. These tags can be added using the google-auto-tagging
    /// or aws-auto-tagging extensions.
    pub ai_tags: Option<Vec<AiTag>>,
}
