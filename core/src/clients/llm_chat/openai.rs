use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

use crate::{
    clients::outbound_http::{OutboundRequestLog, send_text},
    error::{AppError, Result},
};

use super::{
    AvailableLlmModel, LlmClientConfig, LlmMessage, LlmProviderClient,
    urls::{openai_chat_url, openai_models_url},
    util::{dedupe_models, provider_api_error, with_system_prompt},
};

#[derive(Clone, Debug)]
pub(super) struct OpenAiCompatibleClient {
    http: Client,
    config: LlmClientConfig,
}

impl OpenAiCompatibleClient {
    pub(super) fn new(http: Client, config: LlmClientConfig) -> Self {
        Self { http, config }
    }
}

#[derive(Debug, Clone, Serialize)]
struct ChatRequestPayload {
    model: String,
    messages: Vec<LlmMessage>,
    stream: bool,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ChatResponsePayload {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessageContent,
}

#[derive(Debug, Deserialize)]
struct ChatMessageContent {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiModelsResponse {
    data: Vec<OpenAiModelInfo>,
}

#[derive(Debug, Deserialize)]
struct OpenAiModelInfo {
    id: String,
    #[serde(default)]
    name: Option<String>,
}

#[async_trait::async_trait]
impl LlmProviderClient for OpenAiCompatibleClient {
    async fn list_models(&self) -> Result<Vec<AvailableLlmModel>> {
        let url = openai_models_url(&self.config.base_url);
        let response = send_text(
            self.http.get(&url).bearer_auth(&self.config.api_key),
            OutboundRequestLog::new("llm.openai.list_models", Method::GET, &url),
        )
        .await?;

        if !response.status.is_success() {
            return Err(provider_api_error(
                &self.config.provider,
                response.status,
                response.body,
            ));
        }

        let parsed: OpenAiModelsResponse = serde_json::from_str(&response.body)?;
        Ok(dedupe_models(parsed.data.into_iter().map(|model| {
            let name = model.name.unwrap_or_else(|| model.id.clone());
            AvailableLlmModel { id: model.id, name }
        })))
    }

    async fn check_model(&self) -> Result<()> {
        self.check_model_with_chat_completion().await
    }

    async fn chat(&self, messages: Vec<LlmMessage>, system_prompt: Option<&str>) -> Result<String> {
        let payload = ChatRequestPayload {
            model: self.config.model.clone(),
            messages: with_system_prompt(messages, system_prompt),
            stream: false,
            temperature: 0.7,
            max_tokens: 1024,
        };
        let body = serde_json::to_string(&payload)?;
        let url = openai_chat_url(&self.config.base_url);

        let response = send_text(
            self.http
                .post(&url)
                .bearer_auth(&self.config.api_key)
                .json(&payload),
            OutboundRequestLog::new("llm.openai.chat", Method::POST, &url).body(body),
        )
        .await?;

        if !response.status.is_success() {
            return Err(provider_api_error(
                &self.config.provider,
                response.status,
                response.body,
            ));
        }

        let parsed: ChatResponsePayload = serde_json::from_str(&response.body)?;
        parsed
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| {
                AppError::BadGateway(format!("No response from {}", self.config.provider))
            })
    }
}

impl OpenAiCompatibleClient {
    async fn check_model_with_chat_completion(&self) -> Result<()> {
        let payload = ChatRequestPayload {
            model: self.config.model.clone(),
            messages: vec![LlmMessage {
                role: "user".to_string(),
                content: "ping".to_string(),
            }],
            stream: false,
            temperature: 0.0,
            max_tokens: 1,
        };
        let body = serde_json::to_string(&payload)?;
        let url = openai_chat_url(&self.config.base_url);

        let response = send_text(
            self.http
                .post(&url)
                .bearer_auth(&self.config.api_key)
                .json(&payload),
            OutboundRequestLog::new("llm.openai.check_provider", Method::POST, &url).body(body),
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
}
