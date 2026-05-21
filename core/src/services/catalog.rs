use crate::contracts::public::{SupportedExchangePayload, SupportedProviderTypePayload};

pub fn supported_provider_types() -> Vec<SupportedProviderTypePayload> {
    crate::clients::llm_chat::supported_provider_types()
        .iter()
        .map(SupportedProviderTypePayload::from)
        .collect()
}

pub fn supported_exchanges() -> Vec<SupportedExchangePayload> {
    vec![
        SupportedExchangePayload {
            id: "binance".to_string(),
            name: "Binance".to_string(),
            exchange_kind: "cex".to_string(),
        },
        SupportedExchangePayload {
            id: "bybit".to_string(),
            name: "Bybit".to_string(),
            exchange_kind: "cex".to_string(),
        },
        SupportedExchangePayload {
            id: "okx".to_string(),
            name: "OKX".to_string(),
            exchange_kind: "cex".to_string(),
        },
        SupportedExchangePayload {
            id: "bitget".to_string(),
            name: "Bitget".to_string(),
            exchange_kind: "cex".to_string(),
        },
        SupportedExchangePayload {
            id: "kucoin".to_string(),
            name: "KuCoin".to_string(),
            exchange_kind: "cex".to_string(),
        },
        SupportedExchangePayload {
            id: "gate".to_string(),
            name: "Gate".to_string(),
            exchange_kind: "cex".to_string(),
        },
        SupportedExchangePayload {
            id: "hyperliquid".to_string(),
            name: "Hyperliquid".to_string(),
            exchange_kind: "dex".to_string(),
        },
        SupportedExchangePayload {
            id: "aster".to_string(),
            name: "Aster DEX".to_string(),
            exchange_kind: "dex".to_string(),
        },
        SupportedExchangePayload {
            id: "lighter".to_string(),
            name: "Lighter".to_string(),
            exchange_kind: "dex".to_string(),
        },
    ]
}
