use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

use crate::{
    clients::outbound_http::{OutboundRequestLog, send_text},
    error::{AppError, Result},
};

use super::{
    AvailableLlmModel, LlmClientConfig, LlmMessage, LlmProviderClient,
    urls::{anthropic_messages_url, anthropic_model_url, anthropic_models_url},
    util::{dedupe_models, non_empty_text, normalize_message_role, provider_api_error},
};

#[derive(Clone, Debug)]
pub(super) struct AnthropicClient {
    http: Client,
    config: LlmClientConfig,
}

impl AnthropicClient {
    pub(super) fn new(http: Client, config: LlmClientConfig) -> Self {
        Self { http, config }
    }
}

#[derive(Debug, Deserialize)]
struct AnthropicModelsResponse {
    data: Vec<AnthropicModelInfo>,
}

#[derive(Debug, Deserialize)]
struct AnthropicModelInfo {
    id: String,
    #[serde(default, rename = "display_name")]
    display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct AnthropicRequestPayload {
    model: String,
    messages: Vec<LlmMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    temperature: f32,
    max_tokens: u32,
}

#[async_trait::async_trait]
impl LlmProviderClient for AnthropicClient {
    async fn list_models(&self) -> Result<Vec<AvailableLlmModel>> {
        let url = anthropic_models_url(&self.config.base_url);
        let response = send_text(
            self.http
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .header("anthropic-version", "2023-06-01"),
            OutboundRequestLog::new("llm.anthropic.list_models", Method::GET, &url),
        )
        .await?;

        if !response.status.is_success() {
            return Err(provider_api_error(
                &self.config.provider,
                response.status,
                response.body,
            ));
        }

        let parsed: AnthropicModelsResponse = serde_json::from_str(&response.body)?;
        Ok(dedupe_models(parsed.data.into_iter().map(|model| {
            let name = model.display_name.unwrap_or_else(|| model.id.clone());
            AvailableLlmModel { id: model.id, name }
        })))
    }

    async fn check_model(&self) -> Result<()> {
        let url = anthropic_model_url(&self.config.base_url, &self.config.model);
        let response = send_text(
            self.http
                .get(&url)
                .header("x-api-key", &self.config.api_key)
                .header("anthropic-version", "2023-06-01"),
            OutboundRequestLog::new("llm.anthropic.check_model", Method::GET, &url),
        )
        .await?;

        if response.status.is_success() {
            return Ok(());
        }

        Err(provider_api_error(
            &self.config.provider,
            response.status,
            response.body,
        ))
    }

    async fn chat(&self, messages: Vec<LlmMessage>, system_prompt: Option<&str>) -> Result<String> {
        let (messages, system) = anthropic_messages(messages, system_prompt);
        let payload = AnthropicRequestPayload {
            model: self.config.model.clone(),
            messages,
            system,
            temperature: 0.7,
            max_tokens: 1024,
        };
        let body = serde_json::to_string(&payload)?;
        let url = anthropic_messages_url(&self.config.base_url);

        let response = send_text(
            self.http
                .post(&url)
                .header("x-api-key", &self.config.api_key)
                .header("anthropic-version", "2023-06-01")
                .json(&payload),
            OutboundRequestLog::new("llm.anthropic.chat", Method::POST, &url).body(body),
        )
        .await?;

        if !response.status.is_success() {
            return Err(provider_api_error(
                &self.config.provider,
                response.status,
                response.body,
            ));
        }

        #[derive(Deserialize)]
        struct AnthropicResponse {
            content: Vec<AnthropicContent>,
        }

        #[derive(Deserialize)]
        struct AnthropicContent {
            text: Option<String>,
        }

        let parsed: AnthropicResponse = serde_json::from_str(&response.body)?;
        parsed
            .content
            .first()
            .and_then(|content| content.text.clone())
            .ok_or_else(|| {
                AppError::BadGateway(format!("No response from {}", self.config.provider))
            })
    }
}

fn anthropic_messages(
    messages: Vec<LlmMessage>,
    system_prompt: Option<&str>,
) -> (Vec<LlmMessage>, Option<String>) {
    let mut api_messages = Vec::new();
    let mut system_parts = Vec::new();

    if let Some(prompt) = system_prompt.and_then(non_empty_text) {
        system_parts.push(prompt);
    }

    for message in messages {
        let Some(content) = non_empty_text(&message.content) else {
            continue;
        };

        match normalize_message_role(&message.role) {
            "system" => system_parts.push(content),
            "assistant" => api_messages.push(LlmMessage {
                role: "assistant".to_string(),
                content,
            }),
            _ => api_messages.push(LlmMessage {
                role: "user".to_string(),
                content,
            }),
        }
    }

    let system = if system_parts.is_empty() {
        None
    } else {
        Some(system_parts.join("\n\n"))
    };

    (api_messages, system)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anthropic_messages_move_system_content_to_top_level() {
        let (messages, system) = anthropic_messages(
            vec![
                LlmMessage {
                    role: "system".to_string(),
                    content: "system from message".to_string(),
                },
                LlmMessage {
                    role: "user".to_string(),
                    content: "hello".to_string(),
                },
            ],
            Some("system prompt"),
        );

        assert_eq!(
            system.as_deref(),
            Some("system prompt\n\nsystem from message")
        );
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].role, "user");
        assert_eq!(messages[0].content, "hello");
    }
}
