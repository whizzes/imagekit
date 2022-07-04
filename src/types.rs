use serde::Deserialize;

/// Response struct returned on failed requests
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
}
