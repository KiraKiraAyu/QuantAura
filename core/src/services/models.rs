use std::sync::Arc;

use crate::{
    clients::llm_chat::provider_config,
    contracts::models::{
        AvailableModelListPayload, AvailableModelPayload, MessagePayload, ModelConfigPayload,
        ProviderAvailabilityPayload, ProviderConfigInput, SafeModelConfig, SafeProviderConfig,
    },
    error::{AppError, Result},
    repositories::{
        ModelRepo,
        models::{UpsertModelConfig, UpsertProviderConfig},
    },
    services::llm::LlmService,
};

#[derive(Debug, Clone)]
pub struct ModelService {
    repo: Arc<ModelRepo>,
    llm_service: Arc<LlmService>,
}

impl ModelService {
    pub fn new(repo: Arc<ModelRepo>, llm_service: Arc<LlmService>) -> Self {
        Self { repo, llm_service }
    }

    pub async fn list_configs(&self) -> Result<ModelConfigPayload> {
        let rows = self.repo.list_for_user().await.map_err(|err| {
            AppError::Internal(format!("Failed to get LLM configurations: {err}"))
        })?;

        Ok(ModelConfigPayload {
            providers: rows
                .into_iter()
                .map(|row| SafeProviderConfig {
                    id: row.id,
                    name: row.name,
                    provider_type: row.provider_type,
                    enabled: row.enabled != 0,
                    api_key: mask_secret(&row.api_key),
                    base_url: row.base_url,
                    models: row
                        .models
                        .into_iter()
                        .map(|model| SafeModelConfig {
                            id: model.id,
                            provider_id: model.provider_id,
                            name: model.name,
                            model_id: model.model_id,
                        })
                        .collect(),
                })
                .collect(),
        })
    }

    pub async fn update_configs(
        &self,
        providers: Vec<ProviderConfigInput>,
    ) -> Result<MessagePayload> {
        let mut upsert_providers = Vec::new();

        for provider in providers {
            let provider_type = provider.provider_type.trim().to_ascii_lowercase();
            if provider.name.trim().is_empty() {
                return Err(AppError::BadRequest("Provider name is required".into()));
            }
            if !self.llm_service.is_supported_provider(&provider_type) {
                return Err(AppError::BadRequest("Unsupported provider type".into()));
            }

            let mut models = Vec::new();
            for model in provider.models {
                if model.name.trim().is_empty() || model.model_id.trim().is_empty() {
                    return Err(AppError::BadRequest(
                        "Model name and model_id are required".into(),
                    ));
                }
                models.push(UpsertModelConfig {
                    id: model.id,
                    name: model.name.trim().to_string(),
                    model_id: model.model_id.trim().to_string(),
                });
            }

            upsert_providers.push(UpsertProviderConfig {
                id: provider.id,
                name: provider.name.trim().to_string(),
                provider_type,
                enabled: provider.enabled,
                api_key: keep_existing_secret(&provider.api_key),
                base_url: provider.base_url.trim().to_string(),
                models,
            });
        }

        self.repo
            .replace_for_user(upsert_providers)
            .await
            .map_err(|err| {
                AppError::Internal(format!("Failed to update LLM configurations: {err}"))
            })?;

        Ok(MessagePayload {
            message: "LLM configuration updated",
        })
    }

    pub async fn list_available_models(
        &self,
        provider_id: Option<String>,
        provider_type: String,
        api_key: String,
        base_url: String,
    ) -> Result<AvailableModelListPayload> {
        let api_key = self
            .resolve_api_key(provider_id.as_deref(), &api_key)
            .await?;
        let config = self.probe_config(provider_type, api_key, String::new(), base_url)?;
        let client = crate::clients::llm_chat::DefaultLlmClient::new(config)?;
        let models = client.list_models().await?;

        Ok(AvailableModelListPayload {
            models: models
                .into_iter()
                .map(|model| AvailableModelPayload {
                    id: model.id,
                    name: model.name,
                })
                .collect(),
        })
    }

    pub async fn check_provider_availability(
        &self,
        provider_id: Option<String>,
        provider_type: String,
        api_key: String,
        base_url: String,
        model_id: String,
    ) -> Result<ProviderAvailabilityPayload> {
        let api_key = self
            .resolve_api_key(provider_id.as_deref(), &api_key)
            .await?;
        let config = self.probe_config(provider_type, api_key, model_id, base_url)?;
        let client = crate::clients::llm_chat::DefaultLlmClient::new(config)?;
        client.check_provider().await?;

        Ok(ProviderAvailabilityPayload {
            available: true,
            message: "Provider is available",
        })
    }

    /// Resolve the API key a probe should authenticate with.
    ///
    /// Clients only ever receive masked secrets, so a masked or empty value means
    /// "reuse the stored credential" rather than "authenticate with this literal".
    async fn resolve_api_key(&self, provider_id: Option<&str>, incoming: &str) -> Result<String> {
        let incoming = incoming.trim();
        if !incoming.is_empty() && !is_masked(incoming) {
            return Ok(incoming.to_string());
        }

        let stored = match provider_id {
            Some(provider_id) => self.repo.api_key_for(provider_id).await.map_err(|err| {
                AppError::Internal(format!("Failed to load stored provider credentials: {err}"))
            })?,
            None => None,
        };

        Ok(stored.unwrap_or_default().trim().to_string())
    }

    fn probe_config(
        &self,
        provider_type: String,
        api_key: String,
        model_id: String,
        base_url: String,
    ) -> Result<crate::clients::llm_chat::LlmClientConfig> {
        let provider_type = provider_type.trim().to_ascii_lowercase();
        if !self.llm_service.is_supported_provider(&provider_type) {
            return Err(AppError::BadRequest("Unsupported provider type".into()));
        }
        if api_key.trim().is_empty() {
            return Err(AppError::BadRequest("API key is required".into()));
        }

        Ok(provider_config(
            provider_type,
            api_key.trim().to_string(),
            model_id.trim().to_string(),
            base_url.trim().to_string(),
        ))
    }
}

/// Marker embedded in every masked secret returned to clients.
const MASK_MARKER: &str = "****";

/// Mask a stored secret for display purposes.
fn mask_secret(secret: &str) -> String {
    let trimmed = secret.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = trimmed.chars().collect();
    if chars.len() <= 8 {
        return MASK_MARKER.to_string();
    }

    let prefix: String = chars[..3].iter().collect();
    let suffix: String = chars[chars.len() - 4..].iter().collect();
    format!("{prefix}{MASK_MARKER}{suffix}")
}

/// `true` when the value carries the mask marker, i.e. the client echoed back a
/// masked placeholder instead of providing a real secret.
fn is_masked(value: &str) -> bool {
    value.contains(MASK_MARKER)
}

/// Map a client-supplied secret onto a stored-value update. `None` means the
/// request did not carry a new secret, so whatever is stored must be kept.
fn keep_existing_secret(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || is_masked(trimmed) {
        None
    } else {
        Some(trimmed.to_string())
    }
}
