use crate::clients::exchanges::ExchangeSymbolConstraints;

pub fn normalize_order_quantity_by_constraints(raw_qty: f64, c: &ExchangeSymbolConstraints) -> f64 {
    if raw_qty <= 0.0 {
        return 0.0;
    }

    let mut qty = raw_qty;

    if c.step_size > 0.0 {
        qty = (qty / c.step_size).floor() * c.step_size;
    }

    if c.min_qty > 0.0 && qty < c.min_qty {
        return 0.0;
    }

    if c.max_qty > 0.0 && qty > c.max_qty {
        qty = c.max_qty;
        if c.step_size > 0.0 {
            qty = (qty / c.step_size).floor() * c.step_size;
        }
    }

    if qty.is_finite() && qty > 0.0 {
        qty
    } else {
        0.0
    }
}

pub fn normalize_order_price_by_constraints(raw_price: f64, c: &ExchangeSymbolConstraints) -> f64 {
    if raw_price <= 0.0 || !raw_price.is_finite() {
        return 0.0;
    }

    let mut px = raw_price.max(1e-9);
    if c.tick_size > 0.0 {
        px = (px / c.tick_size).floor() * c.tick_size;
    }

    if px.is_finite() && px > 0.0 { px } else { 0.0 }
}
