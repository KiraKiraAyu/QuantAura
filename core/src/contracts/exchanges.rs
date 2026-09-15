use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UpdateExchangeConfigRequest {
    pub exchanges: HashMap<String, ExchangeConfigPatch>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExchangeConfigPatch {
    pub enabled: bool,
    #[serde(default, alias = "apiKey")]
    pub api_key: String,
    #[serde(default, alias = "secretKey")]
    pub secret_key: String,
    #[serde(default)]
    pub passphrase: String,
    #[serde(default)]
    pub testnet: bool,
    #[serde(default, alias = "hyperliquidWalletAddr")]
    pub hyperliquid_wallet_addr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateExchangeRequest {
    #[serde(alias = "exchangeType")]
    pub exchange_type: String,
    #[serde(default, alias = "accountName")]
    pub account_name: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default, alias = "apiKey")]
    pub api_key: String,
    #[serde(default, alias = "secretKey")]
    pub secret_key: String,
    #[serde(default)]
    pub passphrase: String,
    #[serde(default)]
    pub testnet: bool,
    #[serde(default, alias = "hyperliquidWalletAddr")]
    pub hyperliquid_wallet_addr: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SafeExchangeConfig {
    pub id: String,
    #[serde(rename = "exchange_type", alias = "exchangeType")]
    pub exchange_type: String,
    #[serde(rename = "account_name", alias = "accountName")]
    pub account_name: String,
    pub name: String,
    #[serde(rename = "type")]
    pub exchange_kind: String,
    pub enabled: bool,
    pub testnet: bool,
    #[serde(rename = "hyperliquidWalletAddr", alias = "hyperliquid_wallet_addr")]
    pub hyperliquid_wallet_addr: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessagePayload {
    pub message: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateExchangePayload {
    pub message: &'static str,
    pub id: String,
}
