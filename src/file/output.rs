use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum OutputStatus {
    Status(u32),
    Any(Vec<u32>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Output {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OutputStatus>,
    /// Regex
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_contains: Option<String>,
    /// Regex
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_contains: Option<String>,
    /// Regex
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_log_contains: Option<String>,
    #[serde(default, skip_serializing_if = "super::is_false")]
    pub expect_error: bool,
}
