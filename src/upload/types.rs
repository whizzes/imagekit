use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiTag {
    pub name: String,
    pub confidence: f32,
    pub source: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub file_id: String,
    pub name: String,
    pub size: u64,
    pub version_info: VersionInfo,
    pub file_path: String,
    pub url: String,
    pub file_type: String,
    pub height: u64,
    pub width: u64,
    pub thumbnail_url: String,
    pub ai_tags: Option<Vec<AiTag>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
}
