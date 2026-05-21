use crate::error::AppError;

use super::{AvailableLlmModel, LlmMessage};

pub(super) fn with_system_prompt(
    mut messages: Vec<LlmMessage>,
    system_prompt: Option<&str>,
) -> Vec<LlmMessage> {
    if let Some(prompt) = system_prompt.filter(|prompt| !prompt.trim().is_empty()) {
        messages.insert(
            0,
            LlmMessage {
                role: "system".to_string(),
                content: prompt.to_string(),
            },
        );
    }
    messages
}

pub(super) fn normalize_message_role(role: &str) -> &'static str {
    match role.trim().to_ascii_lowercase().as_str() {
        "system" => "system",
        "assistant" | "model" => "assistant",
        _ => "user",
    }
}

pub(super) fn provider_api_error(
    provider: &str,
    status: reqwest::StatusCode,
    body: String,
) -> AppError {
    AppError::BadGateway(format!("{provider} API error: {status} - {body}"))
}

pub(super) fn dedupe_models(
    models: impl IntoIterator<Item = AvailableLlmModel>,
) -> Vec<AvailableLlmModel> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();

    for model in models {
        if model.id.trim().is_empty() || !seen.insert(model.id.clone()) {
            continue;
        }
        out.push(model);
    }

    out
}

pub(super) fn non_empty_text(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
