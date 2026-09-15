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

impl PaginationQuery {
    pub fn sanitized(&self) -> (Option<String>, Option<i64>, Option<i64>) {
        let trader_id = self
            .trader_id
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        let limit = Some(self.limit.unwrap_or(50).clamp(1, 200));
        let offset = Some(self.offset.unwrap_or(0).max(0));
        (trader_id, limit, offset)
    }
}
