use crate::{
    clients::market_data::{fetch_binance_klines, fetch_binance_symbols, normalize_crypto_symbol},
    contracts::public::{
        ExchangeSymbolPayload, ExchangeSymbolsPayload, KlinePayload, KlinesQuery, SymbolsQuery,
    },
    error::{AppError, Result},
};

pub async fn symbols(query: SymbolsQuery) -> Result<ExchangeSymbolsPayload> {
    let exchange = query
        .exchange
        .unwrap_or_else(|| "hyperliquid".to_string())
        .to_ascii_lowercase();

    if exchange == "hyperliquid" || exchange == "hyperliquid-xyz" || exchange == "xyz" {
        let symbols = vec![
            ExchangeSymbolPayload {
                symbol: "BTC".to_string(),
                name: "BTC".to_string(),
                category: "crypto".to_string(),
            },
            ExchangeSymbolPayload {
                symbol: "ETH".to_string(),
                name: "ETH".to_string(),
                category: "crypto".to_string(),
            },
            ExchangeSymbolPayload {
                symbol: "SOL".to_string(),
                name: "SOL".to_string(),
                category: "crypto".to_string(),
            },
            ExchangeSymbolPayload {
                symbol: "AAPL".to_string(),
                name: "AAPL".to_string(),
                category: "stock".to_string(),
            },
            ExchangeSymbolPayload {
                symbol: "TSLA".to_string(),
                name: "TSLA".to_string(),
                category: "stock".to_string(),
            },
            ExchangeSymbolPayload {
                symbol: "EUR".to_string(),
                name: "EUR".to_string(),
                category: "forex".to_string(),
            },
            ExchangeSymbolPayload {
                symbol: "GOLD".to_string(),
                name: "GOLD".to_string(),
                category: "commodity".to_string(),
            },
        ];
        return Ok(ExchangeSymbolsPayload {
            exchange,
            count: symbols.len(),
            symbols,
        });
    }

    if matches!(
        exchange.as_str(),
        "binance" | "bybit" | "okx" | "bitget" | "gate" | "kucoin" | "aster" | "lighter"
    ) {
        let symbols = fetch_binance_symbols().await.unwrap_or_else(|_| {
            vec![
                crate::clients::market_data::MarketSymbol {
                    symbol: "BTCUSDT".to_string(),
                    name: "BTCUSDT".to_string(),
                    category: "crypto".to_string(),
                },
                crate::clients::market_data::MarketSymbol {
                    symbol: "ETHUSDT".to_string(),
                    name: "ETHUSDT".to_string(),
                    category: "crypto".to_string(),
                },
                crate::clients::market_data::MarketSymbol {
                    symbol: "SOLUSDT".to_string(),
                    name: "SOLUSDT".to_string(),
                    category: "crypto".to_string(),
                },
            ]
        });
        let symbols: Vec<ExchangeSymbolPayload> = symbols
            .into_iter()
            .map(ExchangeSymbolPayload::from)
            .collect();
        return Ok(ExchangeSymbolsPayload {
            exchange,
            count: symbols.len(),
            symbols,
        });
    }

    Err(AppError::BadRequest(
        "Unsupported exchange for symbol listing".into(),
    ))
}

pub async fn klines(query: KlinesQuery) -> Result<Vec<KlinePayload>> {
    let symbol = query.symbol.trim().to_uppercase();
    if symbol.is_empty() {
        return Err(AppError::BadRequest("symbol parameter is required".into()));
    }

    let interval = query.interval.unwrap_or_else(|| "5m".to_string());
    let limit = query.limit.unwrap_or(1000).clamp(1, 1500) as usize;
    let exchange = query
        .exchange
        .unwrap_or_else(|| "binance".to_string())
        .to_ascii_lowercase();

    if matches!(
        exchange.as_str(),
        "binance" | "bybit" | "okx" | "bitget" | "gate" | "kucoin" | "aster" | "lighter"
    ) {
        let norm_symbol = normalize_crypto_symbol(&symbol);
        if let Ok(klines) = fetch_binance_klines(&norm_symbol, &interval, limit).await {
            return Ok(klines.into_iter().map(KlinePayload::from).collect());
        }
        return Err(AppError::BadGateway(
            "Market data upstream unavailable for requested exchange".into(),
        ));
    }

    Err(AppError::BadRequest(
        "Unsupported exchange for klines endpoint".into(),
    ))
}
