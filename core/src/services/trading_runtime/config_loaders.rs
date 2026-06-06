use super::service::*;

pub async fn load_trader_runtime_config(
    state: &SharedState,
    user_id: &str,
    trader_id: &str,
) -> Result<Option<TraderRuntimeConfig>, AppError> {
    let Some(row) = state.trading_repo.get_trader(user_id, trader_id).await? else {
        return Ok(None);
    };

    let resolved_model = match state
        .llm_service
        .resolve_for_user(&row.user_id, Some(&row.ai_model_id))
        .await
    {
        Ok(model) => model,
        Err(AppError::BadRequest(_)) => return Ok(None),
        Err(err) => return Err(err),
    };

    Ok(Some(TraderRuntimeConfig {
        trader_id: row.id,
        user_id: row.user_id,
        name: row.name,
        ai_model_id: resolved_model.id,
        ai_model_name: resolved_model.model_id,
        ai_provider_type: resolved_model.provider_type,
        ai_api_key: resolved_model.api_key,
        ai_base_url: resolved_model.base_url,
        exchange_id: row.exchange_id,
        scan_interval_minutes: row.scan_interval_minutes,
        initial_balance: row.initial_balance,
        btc_eth_leverage: row.btc_eth_leverage,
        altcoin_leverage: row.altcoin_leverage,
        trading_symbols: row.trading_symbols,
        custom_prompt: row.custom_prompt,
        override_base_prompt: row.override_base_prompt != 0,
        system_prompt_template: row.system_prompt_template,
    }))
}

pub async fn set_trader_running(
    state: &SharedState,
    trader_id: &str,
    user_id: &str,
    running: bool,
) -> Result<(), AppError> {
    state
        .trading_repo
        .set_trader_running(user_id, trader_id, running, now_i64())
        .await?;
    Ok(())
}

pub async fn load_runtime_execution_context(
    state: &SharedState,
    cfg: &TraderRuntimeConfig,
) -> Result<
    (
        RuntimeExecutionContext,
        Option<Box<dyn LiveExchangeAdapter>>,
    ),
    AppError,
> {
    let Some(row) = state
        .exchange_repo
        .find_runtime_config(&cfg.exchange_id, &cfg.user_id)
        .await?
    else {
        return Ok((
            RuntimeExecutionContext {
                mode: RuntimeExecutionMode::Simulated,
            },
            None,
        ));
    };

    if !row.enabled {
        return Ok((
            RuntimeExecutionContext {
                mode: RuntimeExecutionMode::Simulated,
            },
            None,
        ));
    }

    let api_key = row.api_key;
    let secret_key = row.secret_key;
    let passphrase_raw = row.passphrase;
    let wallet_addr = row.hyperliquid_wallet_addr;
    let testnet = row.testnet;

    if exchange_credentials_missing(
        &row.exchange_type,
        &api_key,
        &secret_key,
        &passphrase_raw,
        &wallet_addr,
    ) {
        return Err(AppError::InvalidExchangeConfig(format!(
            "enabled exchange credentials are incomplete for trader={}",
            cfg.trader_id
        )));
    }

    let credentials = ExchangeCredentials {
        api_key,
        secret_key,
        passphrase: if passphrase_raw.trim().is_empty() {
            None
        } else {
            Some(passphrase_raw)
        },
        wallet_addr: if wallet_addr.trim().is_empty() {
            None
        } else {
            Some(wallet_addr)
        },
        testnet,
    };

    let adapter = create_exchange_adapter(&row.exchange_type, credentials)?;
    if let Err(err) = adapter.ping().await {
        return Err(AppError::BadGateway(format!(
            "enabled exchange ping failed for trader={}: {}",
            cfg.trader_id, err
        )));
    }

    Ok((
        RuntimeExecutionContext {
            mode: RuntimeExecutionMode::LiveExchange,
        },
        Some(adapter),
    ))
}

pub fn exchange_credentials_missing(
    exchange_type: &str,
    api_key: &str,
    secret_key: &str,
    passphrase: &str,
    wallet_addr: &str,
) -> bool {
    match exchange_type.trim().to_ascii_lowercase().as_str() {
        "hyperliquid" => wallet_addr.trim().is_empty() || secret_key.trim().is_empty(),
        "okx" | "bitget" => {
            api_key.trim().is_empty()
                || secret_key.trim().is_empty()
                || passphrase.trim().is_empty()
        }
        _ => api_key.trim().is_empty() || secret_key.trim().is_empty(),
    }
}

pub fn parse_symbols(raw: &str) -> Vec<String> {
    let mut out = raw
        .split(|c: char| c == ',' || c.is_whitespace() || c == ';' || c == '|')
        .map(|s| s.trim().to_uppercase())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if out.is_empty() {
        out = vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()];
    }

    out.sort();
    out.dedup();
    out
}

pub fn leverage_for_symbol(cfg: &TraderRuntimeConfig, symbol: &str) -> i64 {
    let is_major = symbol.contains("BTC") || symbol.contains("ETH");
    if is_major {
        cfg.btc_eth_leverage.clamp(1, 50)
    } else {
        cfg.altcoin_leverage.clamp(1, 50)
    }
}

pub fn liquidation_price(side: &str, entry: f64, leverage: i64) -> f64 {
    let lv = leverage.max(1) as f64;
    if side == "LONG" {
        (entry * (1.0 - (0.9 / lv))).max(0.0001)
    } else {
        (entry * (1.0 + (0.9 / lv))).max(0.0001)
    }
}

pub fn default_price_for_symbol(symbol: &str) -> f64 {
    match symbol {
        s if s.contains("BTC") => 65_000.0,
        s if s.contains("ETH") => 3_200.0,
        s if s.contains("SOL") => 160.0,
        s if s.contains("BNB") => 560.0,
        s if s.contains("XRP") => 0.65,
        s if s.contains("DOGE") => 0.16,
        _ => 50.0,
    }
}

pub fn deterministic_noise(seed: u64) -> f64 {
    let mut x = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    let frac = (x as f64 / u64::MAX as f64).clamp(0.0, 1.0);
    (frac * 2.0) - 1.0
}

pub fn hash_str(s: &str) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

pub fn parse_f64(v: &str) -> f64 {
    v.parse::<f64>().unwrap_or(0.0)
}

pub fn now_i64() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn now_u64() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exchange_credentials_missing_matches_exchange_requirements() {
        assert!(exchange_credentials_missing("okx", "key", "secret", "", ""));
        assert!(!exchange_credentials_missing(
            "bitget",
            "key",
            "secret",
            "passphrase",
            ""
        ));
        assert!(exchange_credentials_missing(
            "hyperliquid",
            "",
            "private-key",
            "",
            ""
        ));
        assert!(!exchange_credentials_missing(
            "hyperliquid",
            "",
            "private-key",
            "",
            "0xabc"
        ));
        assert!(exchange_credentials_missing(
            "binance", "", "secret", "", ""
        ));
        assert!(!exchange_credentials_missing(
            "aster", "key", "secret", "", ""
        ));
    }
}
