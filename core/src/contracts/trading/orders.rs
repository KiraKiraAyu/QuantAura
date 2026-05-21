use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct OrderPayload {
    pub id: String,
    pub exchange_order_id: String,
    pub client_order_id: String,
    pub symbol: String,
    pub side: String,
    pub position_side: String,
    pub order_type: String,
    pub status: String,
    pub price: f64,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub avg_fill_price: Option<f64>,
    pub reduce_only: bool,
    pub time_in_force: String,
    pub placed_at: i64,
    pub updated_at: i64,
    pub closed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderListPayload {
    pub trader_id: String,
    pub items: Vec<OrderPayload>,
    pub count: usize,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FillPayload {
    pub id: String,
    pub exchange_trade_id: String,
    pub symbol: String,
    pub side: String,
    pub price: f64,
    pub quantity: f64,
    pub fee: f64,
    pub fee_asset: String,
    pub realized_pnl: f64,
    pub executed_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FillListPayload {
    pub trader_id: String,
    pub order_id: String,
    pub items: Vec<FillPayload>,
    pub count: usize,
}
