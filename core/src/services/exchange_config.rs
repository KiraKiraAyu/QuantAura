use std::sync::Arc;

use crate::{
    contracts::exchanges::{
        CreateExchangePayload, CreateExchangeRequest, MessagePayload, SafeExchangeConfig,
        UpdateExchangeConfigRequest,
    },
    error::{AppError, Result},
    repositories::{
        ExchangeRepo,
        exchanges::{CreateExchangeAccount, UpdateExchangeAccount},
    },
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ExchangeConfigService {
    repo: Arc<ExchangeRepo>,
}

impl ExchangeConfigService {
    pub fn new(repo: Arc<ExchangeRepo>) -> Self {
        Self { repo }
    }

    pub async fn list_configs(&self, user_id: &str) -> Result<Vec<SafeExchangeConfig>> {
        let rows =
            self.repo.list_for_user(user_id).await.map_err(|err| {
                AppError::Internal(format!("Failed to get exchange configs: {err}"))
            })?;

        Ok(rows
            .into_iter()
            .map(|row| SafeExchangeConfig {
                id: row.id,
                exchange_type: row.exchange_type,
                account_name: row.account_name,
                name: row.name,
                exchange_kind: row.exchange_kind,
                enabled: row.enabled != 0,
                testnet: row.testnet != 0,
                hyperliquid_wallet_addr: row.hyperliquid_wallet_addr,
                aster_user: row.aster_user,
                aster_signer: row.aster_signer,
                lighter_wallet_addr: row.lighter_wallet_addr,
                lighter_api_key_index: row.lighter_api_key_index,
            })
            .collect())
    }

    pub async fn create_exchange(
        &self,
        user_id: &str,
        request: CreateExchangeRequest,
    ) -> Result<CreateExchangePayload> {
        let exchange_type = request.exchange_type.trim().to_ascii_lowercase();
        if !is_supported_exchange_type(&exchange_type) {
            return Err(AppError::BadRequest("Invalid exchange type".into()));
        }

        let account_name = if request.account_name.trim().is_empty() {
            "Default".to_string()
        } else {
            request.account_name.trim().to_string()
        };

        let (name, exchange_kind) = exchange_name_and_type(&exchange_type);
        let id = Uuid::now_v7().to_string();
        let now = now_ts();

        self.repo
            .create(CreateExchangeAccount {
                id: id.clone(),
                exchange_type,
                account_name,
                user_id: user_id.trim().to_string(),
                name: name.to_string(),
                exchange_kind: exchange_kind.to_string(),
                enabled: request.enabled,
                api_key: request.api_key.trim().to_string(),
                secret_key: request.secret_key.trim().to_string(),
                passphrase: request.passphrase.trim().to_string(),
                testnet: request.testnet,
                hyperliquid_wallet_addr: request.hyperliquid_wallet_addr.trim().to_string(),
                aster_user: request.aster_user.trim().to_string(),
                aster_signer: request.aster_signer.trim().to_string(),
                aster_private_key: request.aster_private_key.trim().to_string(),
                lighter_wallet_addr: request.lighter_wallet_addr.trim().to_string(),
                lighter_private_key: request.lighter_private_key.trim().to_string(),
                lighter_api_key_private_key: request.lighter_api_key_private_key.trim().to_string(),
                lighter_api_key_index: request.lighter_api_key_index,
                created_at: now,
                updated_at: now,
            })
            .await
            .map_err(|err| {
                AppError::Internal(format!("Failed to create exchange account: {err}"))
            })?;

        Ok(CreateExchangePayload {
            message: "Exchange account created",
            id,
        })
    }

    pub async fn update_configs(
        &self,
        user_id: &str,
        request: UpdateExchangeConfigRequest,
    ) -> Result<MessagePayload> {
        let now = now_ts();

        for (exchange_id, patch) in request.exchanges {
            let existing = self
                .repo
                .find_secrets(&exchange_id, user_id)
                .await
                .map_err(|err| {
                    AppError::Internal(format!("Failed to load exchange secrets: {err}"))
                })?;

            let Some(existing) = existing else {
                continue;
            };

            self.repo
                .update(
                    &exchange_id,
                    user_id,
                    UpdateExchangeAccount {
                        enabled: patch.enabled,
                        api_key: keep_or_new(existing.api_key, &patch.api_key),
                        secret_key: keep_or_new(existing.secret_key, &patch.secret_key),
                        passphrase: keep_or_new(existing.passphrase, &patch.passphrase),
                        testnet: patch.testnet,
                        hyperliquid_wallet_addr: patch.hyperliquid_wallet_addr.trim().to_string(),
                        aster_user: patch.aster_user.trim().to_string(),
                        aster_signer: patch.aster_signer.trim().to_string(),
                        aster_private_key: keep_or_new(
                            existing.aster_private_key,
                            &patch.aster_private_key,
                        ),
                        lighter_wallet_addr: patch.lighter_wallet_addr.trim().to_string(),
                        lighter_private_key: keep_or_new(
                            existing.lighter_private_key,
                            &patch.lighter_private_key,
                        ),
                        lighter_api_key_private_key: keep_or_new(
                            existing.lighter_api_key_private_key,
                            &patch.lighter_api_key_private_key,
                        ),
                        lighter_api_key_index: patch.lighter_api_key_index,
                        updated_at: now,
                    },
                )
                .await
                .map_err(|err| {
                    AppError::Internal(format!("Failed to update exchange config: {err}"))
                })?;
        }

        Ok(MessagePayload {
            message: "Exchange configuration updated",
        })
    }

    pub async fn delete_exchange(
        &self,
        user_id: &str,
        exchange_id: &str,
    ) -> Result<MessagePayload> {
        if exchange_id.trim().is_empty() {
            return Err(AppError::BadRequest("Exchange ID is required".into()));
        }

        if self
            .repo
            .find_trader_usage(user_id, exchange_id)
            .await
            .map_err(|err| AppError::Internal(format!("Failed to check trader usage: {err}")))?
            .is_some()
        {
            return Err(AppError::Conflict(
                "Cannot delete exchange account that is in use by traders".into(),
            ));
        }

        let deleted = self
            .repo
            .delete(exchange_id, user_id)
            .await
            .map_err(|err| {
                AppError::Internal(format!("Failed to delete exchange account: {err}"))
            })?;

        if deleted == 0 {
            return Err(AppError::NotFound("Exchange not found".into()));
        }

        Ok(MessagePayload {
            message: "Exchange account deleted",
        })
    }
}

fn now_ts() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}

fn keep_or_new(existing: String, incoming: &str) -> String {
    let trimmed = incoming.trim();
    if trimmed.is_empty() {
        existing
    } else {
        trimmed.to_string()
    }
}

fn is_supported_exchange_type(exchange_type: &str) -> bool {
    matches!(
        exchange_type,
        "binance"
            | "bybit"
            | "okx"
            | "bitget"
            | "kucoin"
            | "gate"
            | "hyperliquid"
            | "aster"
            | "lighter"
    )
}

fn exchange_name_and_type(exchange_type: &str) -> (&'static str, &'static str) {
    match exchange_type {
        "binance" => ("Binance Futures", "cex"),
        "bybit" => ("Bybit Futures", "cex"),
        "okx" => ("OKX Futures", "cex"),
        "bitget" => ("Bitget Futures", "cex"),
        "kucoin" => ("KuCoin Futures", "cex"),
        "gate" => ("Gate.io Futures", "cex"),
        "hyperliquid" => ("Hyperliquid", "dex"),
        "aster" => ("Aster DEX", "dex"),
        "lighter" => ("Lighter", "dex"),
        _ => ("Unknown Exchange", "cex"),
    }
}
