use super::service::*;

pub async fn refresh_market_from_exchange(
    symbols: &[String],
    market: &mut HashMap<String, MarketState>,
    adapter: &dyn LiveExchangeAdapter,
) {
    for sym in symbols {
        match get_price_with_retry(adapter, sym).await {
            Ok(px) if px > 0.0 => {
                let entry = market.entry(sym.clone()).or_insert(MarketState {
                    price: px,
                    prev_price: px,
                    volatility: 0.01,
                });
                entry.prev_price = entry.price;
                entry.price = px;
                let drift = if entry.prev_price.abs() > f64::EPSILON {
                    ((entry.price - entry.prev_price) / entry.prev_price).abs()
                } else {
                    0.0
                };
                entry.volatility = (entry.volatility * 0.9 + drift * 0.1).clamp(0.002, 0.05);
            }
            Ok(_) => {}
            Err(err) => {
                warn!("failed to fetch live price symbol={} err={}", sym, err);
            }
        }
    }
}
