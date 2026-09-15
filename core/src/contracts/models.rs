use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UpdateModelConfigRequest {
    #[serde(default)]
    pub providers: Vec<ProviderConfigInput>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelProviderProbeRequest {
    /// Stored provider the probe refers to. Lets the backend resolve a masked
    /// `apiKey` back to the credential it already holds.
    #[serde(default, rename = "providerId", alias = "provider_id")]
    pub provider_id: Option<String>,
    #[serde(rename = "providerType", alias = "provider_type")]
    pub provider_type: String,
    #[serde(default, rename = "apiKey", alias = "api_key")]
    pub api_key: String,
    #[serde(default, rename = "baseUrl", alias = "base_url")]
    pub base_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderAvailabilityRequest {
    #[serde(default, rename = "providerId", alias = "provider_id")]
    pub provider_id: Option<String>,
    #[serde(rename = "providerType", alias = "provider_type")]
    pub provider_type: String,
    #[serde(default, rename = "apiKey", alias = "api_key")]
    pub api_key: String,
    #[serde(default, rename = "baseUrl", alias = "base_url")]
    pub base_url: String,
    #[serde(default, rename = "modelId", alias = "model_id")]
    pub model_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderConfigInput {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "providerType", alias = "provider_type")]
    pub provider_type: String,
    pub enabled: bool,
    #[serde(default, rename = "apiKey", alias = "api_key")]
    pub api_key: String,
    #[serde(default, rename = "baseUrl", alias = "base_url")]
    pub base_url: String,
    #[serde(default)]
    pub models: Vec<ModelConfigInput>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelConfigInput {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "modelId", alias = "model_id")]
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModelConfigPayload {
    pub providers: Vec<SafeProviderConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AvailableModelPayload {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AvailableModelListPayload {
    pub models: Vec<AvailableModelPayload>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProviderAvailabilityPayload {
    pub available: bool,
    pub message: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct SafeProviderConfig {
    pub id: String,
    pub name: String,
    #[serde(rename = "providerType", alias = "provider_type")]
    pub provider_type: String,
    pub enabled: bool,
    #[serde(rename = "apiKey", alias = "api_key")]
    pub api_key: String,
    #[serde(rename = "baseUrl", alias = "base_url")]
    pub base_url: String,
    pub models: Vec<SafeModelConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SafeModelConfig {
    pub id: String,
    #[serde(rename = "providerId", alias = "provider_id")]
    pub provider_id: String,
    pub name: String,
    #[serde(rename = "modelId", alias = "model_id")]
    pub model_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessagePayload {
    pub message: &'static str,
}
