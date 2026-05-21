use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TraderQuery {
    pub trader_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub trader_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
