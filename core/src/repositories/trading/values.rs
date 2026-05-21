use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

pub(super) fn decimal_from_f64(value: f64) -> Decimal {
    Decimal::from_f64_retain(value).unwrap_or(Decimal::ZERO)
}

pub(super) fn decimal_to_f64(value: &Decimal) -> f64 {
    value.to_f64().unwrap_or(0.0)
}
