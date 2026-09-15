use std::sync::Arc;

use crate::{error::Result, repositories::AppSettingsRepo};

pub const TIMEZONE_KEY: &str = "timezone";

#[derive(Debug, Clone)]
pub struct SystemService {
    repo: Arc<AppSettingsRepo>,
}

impl SystemService {
    pub fn new(repo: Arc<AppSettingsRepo>) -> Self {
        Self { repo }
    }

    pub async fn get_timezone(&self) -> Result<Option<String>> {
        Ok(self.repo.get(TIMEZONE_KEY).await?)
    }

    pub async fn update_timezone(&self, timezone: Option<&str>) -> Result<Option<String>> {
        if let Some(tz) = timezone {
            let tz = tz.trim();
            if tz.is_empty() {
                self.repo.delete(TIMEZONE_KEY).await?;
            } else {
                self.repo.set(TIMEZONE_KEY, tz).await?;
            }
        }
        Ok(self.repo.get(TIMEZONE_KEY).await?)
    }
}
