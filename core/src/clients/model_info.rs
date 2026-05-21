use std::time::Duration;

use async_trait::async_trait;
use reqwest::Method;
use rust_decimal::Decimal;
use serde::Deserialize;
use tracing::warn;

use crate::{
    clients::outbound_http::{OutboundRequestLog, send_text},
    error::{AppError, Result},
};

#[derive(Debug, Clone)]
pub struct ModelInfoProviderConfig {
    pub api_key: String,
    pub base_url: String,
}

#[async_trait]
pub trait ModelInfoClient: Send + Sync {
    async fn fetch_prices(
        &self,
        provider: &ModelInfoProviderConfig,
        model_id: &str,
    ) -> Result<(Decimal, Decimal)>;

    async fn check_connectivity(&self, provider: &ModelInfoProviderConfig) -> Result<()>;
}

#[derive(Clone)]
pub struct DefaultModelInfoClient {
    http: reqwest::Client,
}

impl Default for DefaultModelInfoClient {
    fn default() -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { http }
    }
}

#[derive(Debug, Deserialize)]
struct PricingResponse {
    input_price_per_million: Decimal,
    output_price_per_million: Decimal,
}

#[async_trait]
impl ModelInfoClient for DefaultModelInfoClient {
    async fn fetch_prices(
        &self,
        provider: &ModelInfoProviderConfig,
        model_id: &str,
    ) -> Result<(Decimal, Decimal)> {
        let base = provider.base_url.trim_end_matches('/');
        let candidates = [
            format!("{}/pricing/models/{}", base, model_id),
            format!("{}/v1/pricing/models/{}", base, model_id),
        ];

        for url in candidates {
            let mut req = self.http.get(&url);
            if !provider.api_key.trim().is_empty() {
                req = req.bearer_auth(&provider.api_key);
            }

            if let Ok(resp) = send_text(
                req,
                OutboundRequestLog::new("model_info.fetch_prices", Method::GET, &url),
            )
            .await
            {
                if resp.status.is_success() {
                    let parsed: PricingResponse =
                        serde_json::from_str(&resp.body).map_err(|err| {
                            AppError::Internal(format!("Failed to parse pricing response: {err}"))
                        })?;
                    return Ok((
                        parsed.input_price_per_million,
                        parsed.output_price_per_million,
                    ));
                }
            }
        }

        warn!(
            "Could not fetch prices for model {}, defaulting to 0: cannot reach pricing endpoints",
            model_id
        );
        Ok((Decimal::ZERO, Decimal::ZERO))
    }

    async fn check_connectivity(&self, provider: &ModelInfoProviderConfig) -> Result<()> {
        let base = provider.base_url.trim_end_matches('/');
        let url = if base.ends_with("/v1") {
            format!("{}/models", base)
        } else {
            format!("{}/v1/models", base)
        };

        let mut req = self.http.get(&url);
        if !provider.api_key.trim().is_empty() {
            req = req.bearer_auth(&provider.api_key);
        }

        let resp = send_text(
            req,
            OutboundRequestLog::new("model_info.check_connectivity", Method::GET, &url),
        )
        .await
        .map_err(|err| AppError::Internal(format!("Network error: {err}")))?;

        if resp.status.is_success() {
            Ok(())
        } else {
            Err(AppError::BadRequest(format!(
                "Provider check failed with status: {}",
                resp.status
            )))
        }
    }
}
