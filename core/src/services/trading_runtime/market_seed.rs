use super::service::*;

pub async fn seed_market(
    cfg: &TraderRuntimeConfig,
    symbols: &[String],
) -> Result<HashMap<String, MarketState>, AppError> {
    let mut market = HashMap::new();
    for s in symbols {
        let base = default_price_for_symbol(s);
        let tilt = 1.0 + deterministic_noise(hash_str(&(cfg.trader_id.clone() + s))) * 0.03;
        let price = (base * tilt).max(0.0001);
        market.insert(
            s.clone(),
            MarketState {
                price,
                prev_price: price,
                volatility: 0.008 + (deterministic_noise(hash_str(s)).abs() * 0.01),
            },
        );
    }
    Ok(market)
}

pub fn advance_market(
    cfg: &TraderRuntimeConfig,
    seed_time: u64,
    symbols: &[String],
    market: &mut HashMap<String, MarketState>,
) {
    for s in symbols {
        let state = market.entry(s.clone()).or_insert(MarketState {
            price: default_price_for_symbol(s),
            prev_price: default_price_for_symbol(s),
            volatility: 0.01,
        });

        let seed = seed_time ^ hash_str(&(cfg.trader_id.clone() + s));
        let noise = deterministic_noise(seed);
        let cyc = ((seed_time as f64 / 300.0) + (hash_str(s) % 73) as f64).sin() * 0.0009;
        let shock = noise * state.volatility * 0.7;

        state.prev_price = state.price;
        state.price = (state.price * (1.0 + cyc + shock)).max(0.0001);
        state.volatility = (state.volatility * 0.92 + noise.abs() * 0.02).clamp(0.003, 0.04);
    }
}
