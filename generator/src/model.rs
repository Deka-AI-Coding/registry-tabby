use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    /// Name of the model
    pub name: String,

    /// Url to main model page
    pub provider_url: String,

    /// Url to license
    pub license_url: String,

    /// Template for code completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<String>,

    /// Template for chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_completion: Option<String>,

    /// urls to download the model
    pub urls: Vec<String>,

    /// Checksum of the model
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelInput {
    /// Template for code completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<String>,

    /// Template for chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_completion: Option<String>,

    /// Name of the model
    pub name: String,

    /// Tags to generate
    pub tags: Vec<String>,
}
