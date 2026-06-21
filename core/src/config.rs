use std::{
    fmt, fs,
    net::{IpAddr, SocketAddr},
    path::Path,
};

use envconfig::Envconfig;
use jsonwebtoken::{DecodingKey, EncodingKey};

macro_rules! ensure {
    ($cond:expr, $msg:expr) => {
        assert!($cond, "Invalid config: {}", $msg);
    };
}

macro_rules! validate_all {
    ($($cond:expr => $msg:expr),+ $(,)?) => {
        $(ensure!($cond, $msg);)+
    };
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app: AppMetadataConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub auth: AuthConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub live: LiveRuntimeConfig,
    pub runtime_alerts: RuntimeAlertConfig,
}

#[derive(Debug, Clone, Envconfig)]
struct EnvAppConfig {
    #[envconfig(nested)]
    app: AppMetadataConfig,
    #[envconfig(nested)]
    server: ServerConfig,
    #[envconfig(nested)]
    jwt: JwtPathConfig,
    #[envconfig(nested)]
    auth: AuthConfig,
    #[envconfig(nested)]
    database: DatabaseConfig,
    #[envconfig(nested)]
    logging: LoggingConfig,
    #[envconfig(nested)]
    live: LiveRuntimeConfig,
    #[envconfig(nested)]
    runtime_alerts: RuntimeAlertConfig,
}

impl EnvAppConfig {
    fn into_app_config(self) -> Result<AppConfig, String> {
        Ok(AppConfig {
            app: self.app,
            server: self.server,
            jwt: JwtConfig::from_key_paths(
                &self.jwt.private_key_path,
                &self.jwt.public_key_path,
                self.jwt.issuer,
                self.jwt.ttl_secs,
            )?,
            auth: self.auth,
            database: self.database,
            logging: self.logging,
            live: self.live,
            runtime_alerts: self.runtime_alerts,
        })
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        load_dotenv();

        let env_config = EnvAppConfig::init_from_env()
            .unwrap_or_else(|err| panic!("Failed to load config from env: {err}"));
        let config = env_config
            .into_app_config()
            .unwrap_or_else(|err| panic!("Failed to load JWT keys: {err}"));

        config.validate();

        config
    }

    pub fn validate(&self) {
        self.server.validate();
        self.jwt.validate();
        self.auth.validate();
        self.database.validate();
        self.live.validate();
        self.runtime_alerts.validate();
    }

    pub fn server_addr(&self) -> SocketAddr {
        self.server.socket_addr()
    }
}

fn load_dotenv() {
    if let Ok(path) = std::env::var("ENV_FILE") {
        dotenvy::from_path(&path)
            .unwrap_or_else(|err| panic!("Failed to load env file from {path}: {err}"));
        return;
    }

    for path in ["../.env", ".env"] {
        if Path::new(path).exists() {
            dotenvy::from_path(path)
                .unwrap_or_else(|err| panic!("Failed to load env file from {path}: {err}"));
            return;
        }
    }
}

#[derive(Debug, Clone, Envconfig)]
pub struct AppMetadataConfig {
    #[envconfig(from = "APP_NAME", default = "quantaura")]
    pub name: String,
    #[envconfig(from = "ENV", default = "development")]
    pub environment: String,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub issuer: String,
    pub ttl_secs: u64,
}

impl fmt::Debug for JwtConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JwtConfig")
            .field("issuer", &self.issuer)
            .field("ttl_secs", &self.ttl_secs)
            .finish_non_exhaustive()
    }
}

impl JwtConfig {
    fn from_key_paths(
        private_key_path: impl AsRef<Path>,
        public_key_path: impl AsRef<Path>,
        issuer: String,
        ttl_secs: u64,
    ) -> Result<Self, String> {
        let private_key_path = private_key_path.as_ref();
        let public_key_path = public_key_path.as_ref();

        if private_key_path.as_os_str().is_empty() {
            return Err("JWT_PRIVATE_KEY_PATH cannot be empty".into());
        }

        if public_key_path.as_os_str().is_empty() {
            return Err("JWT_PUBLIC_KEY_PATH cannot be empty".into());
        }

        if issuer.trim().is_empty() {
            return Err("JWT_ISSUER cannot be empty".into());
        }

        if ttl_secs == 0 {
            return Err("JWT_TTL_SECS cannot be 0".into());
        }

        let private_key = fs::read(private_key_path).map_err(|err| {
            format!(
                "failed to read JWT private key from {}: {err}",
                private_key_path.display()
            )
        })?;
        let public_key = fs::read(public_key_path).map_err(|err| {
            format!(
                "failed to read JWT public key from {}: {err}",
                public_key_path.display()
            )
        })?;

        let encoding_key = EncodingKey::from_rsa_pem(&private_key).map_err(|err| {
            format!(
                "invalid JWT private key at {}: {err}",
                private_key_path.display()
            )
        })?;
        let decoding_key = DecodingKey::from_rsa_pem(&public_key).map_err(|err| {
            format!(
                "invalid JWT public key at {}: {err}",
                public_key_path.display()
            )
        })?;

        Ok(Self {
            encoding_key,
            decoding_key,
            issuer,
            ttl_secs,
        })
    }

    fn validate(&self) {
        validate_all! {
            !self.issuer.trim().is_empty() => "JWT_ISSUER cannot be empty",
            self.ttl_secs != 0 => "JWT_TTL_SECS cannot be 0",
        }
    }
}

#[derive(Debug, Clone, Envconfig)]
struct JwtPathConfig {
    #[envconfig(from = "JWT_PRIVATE_KEY_PATH")]
    private_key_path: String,
    #[envconfig(from = "JWT_PUBLIC_KEY_PATH")]
    public_key_path: String,
    #[envconfig(from = "JWT_ISSUER", default = "quantaura")]
    issuer: String,
    #[envconfig(from = "JWT_TTL_SECS", default = "86400")]
    ttl_secs: u64,
}

#[derive(Debug, Clone, Envconfig)]
pub struct ServerConfig {
    #[envconfig(from = "HOST", default = "0.0.0.0")]
    pub host: IpAddr,
    #[envconfig(from = "PORT", default = "8080")]
    pub port: u16,
    #[envconfig(from = "CORS_ALLOW_ORIGIN", default = "*")]
    pub cors_allow_origin: String,
    #[envconfig(from = "REQUEST_TIMEOUT_SECS", default = "15")]
    pub request_timeout_secs: u64,
}

impl ServerConfig {
    fn validate(&self) {
        validate_all! {
            self.port != 0 => "PORT cannot be 0",
        }
    }

    fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

#[derive(Debug, Clone, Envconfig)]
pub struct AuthConfig {
    #[envconfig(from = "REGISTRATION_ENABLED", default = "true")]
    pub registration_enabled: bool,
    #[envconfig(from = "MAX_USERS", default = "0")]
    pub max_users: usize,
    #[envconfig(from = "TOKEN_BLACKLIST_MAX_ENTRIES", default = "100000")]
    pub token_blacklist_max_entries: usize,
}

impl AuthConfig {
    fn validate(&self) {
        validate_all! {
            self.token_blacklist_max_entries != 0 => "TOKEN_BLACKLIST_MAX_ENTRIES cannot be 0",
        }
    }
}

#[derive(Debug, Clone, Envconfig)]
pub struct DatabaseConfig {
    #[envconfig(from = "DB_URL", default = "sqlite://data/quantaura.db")]
    pub url: String,
}

impl DatabaseConfig {
    fn validate(&self) {
        validate_all! {
            !self.url.trim().is_empty() => "DB_URL cannot be empty",
        }
    }
}

#[derive(Debug, Clone, Envconfig)]
pub struct LoggingConfig {
    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub level: String,
}

#[derive(Debug, Clone, Envconfig)]
pub struct LiveRuntimeConfig {
    #[envconfig(from = "LIVE_OPEN_ORDER_COOLDOWN_SECS", default = "90")]
    pub open_order_cooldown_secs: u64,
    #[envconfig(from = "LIVE_STALE_OPEN_ORDER_CANCEL_SECS", default = "180")]
    pub stale_open_order_cancel_secs: u64,
    #[envconfig(from = "LIVE_STALE_LIMIT_ORDER_REPLACE_SECS", default = "90")]
    pub stale_limit_order_replace_secs: u64,
    #[envconfig(from = "LIVE_REPLACE_MAX_ATTEMPTS_PER_WINDOW", default = "3")]
    pub replace_max_attempts_per_window: u64,
    #[envconfig(from = "LIVE_REPLACE_ATTEMPT_WINDOW_SECS", default = "180")]
    pub replace_attempt_window_secs: u64,
    #[envconfig(from = "LIVE_SUBMITTED_INTENT_RECONCILE_SECS", default = "300")]
    pub submitted_intent_reconcile_secs: u64,
    #[envconfig(from = "LIVE_RISK_SOFT_DRAWDOWN_PCT", default = "15.0")]
    pub risk_soft_drawdown_pct: f64,
    #[envconfig(from = "LIVE_RISK_MEDIUM_DRAWDOWN_PCT", default = "25.0")]
    pub risk_medium_drawdown_pct: f64,
    #[envconfig(from = "LIVE_RISK_HARD_DRAWDOWN_PCT", default = "35.0")]
    pub risk_hard_drawdown_pct: f64,
    #[envconfig(from = "LIVE_RISK_SOFT_MARGIN_RATIO", default = "0.70")]
    pub risk_soft_margin_ratio: f64,
    #[envconfig(from = "LIVE_RISK_MEDIUM_MARGIN_RATIO", default = "0.82")]
    pub risk_medium_margin_ratio: f64,
    #[envconfig(from = "LIVE_RISK_HARD_MARGIN_RATIO", default = "0.90")]
    pub risk_hard_margin_ratio: f64,
    #[envconfig(from = "LIVE_RISK_SOFT_OPEN_COOLDOWN_MULTIPLIER", default = "2")]
    pub risk_soft_open_cooldown_multiplier: u64,
    #[envconfig(from = "LIVE_RISK_MEDIUM_REDUCE_POSITIONS_COUNT", default = "1")]
    pub risk_medium_reduce_positions_count: u64,
    #[envconfig(from = "LIVE_RISK_HARD_CLOSE_WORST_POSITIONS_COUNT", default = "2")]
    pub risk_hard_close_worst_positions_count: u64,
}

impl LiveRuntimeConfig {
    fn validate(&self) {
        validate_all! {
            self.open_order_cooldown_secs != 0 => "LIVE_OPEN_ORDER_COOLDOWN_SECS cannot be 0",
            self.stale_open_order_cancel_secs != 0 => "LIVE_STALE_OPEN_ORDER_CANCEL_SECS cannot be 0",
            self.stale_limit_order_replace_secs != 0 => "LIVE_STALE_LIMIT_ORDER_REPLACE_SECS cannot be 0",
            self.replace_max_attempts_per_window != 0 => "LIVE_REPLACE_MAX_ATTEMPTS_PER_WINDOW cannot be 0",
            self.replace_attempt_window_secs != 0 => "LIVE_REPLACE_ATTEMPT_WINDOW_SECS cannot be 0",
            self.submitted_intent_reconcile_secs != 0 => "LIVE_SUBMITTED_INTENT_RECONCILE_SECS cannot be 0",
            self.risk_soft_drawdown_pct > 0.0
                && self.risk_medium_drawdown_pct > 0.0
                && self.risk_hard_drawdown_pct > 0.0
                => "live risk drawdown thresholds must be > 0",
            self.risk_soft_drawdown_pct <= self.risk_medium_drawdown_pct
                && self.risk_medium_drawdown_pct <= self.risk_hard_drawdown_pct
                => "live risk drawdown thresholds must satisfy soft <= medium <= hard",
            self.risk_soft_margin_ratio > 0.0
                && self.risk_medium_margin_ratio > 0.0
                && self.risk_hard_margin_ratio > 0.0
                && self.risk_soft_margin_ratio <= 1.0
                && self.risk_medium_margin_ratio <= 1.0
                && self.risk_hard_margin_ratio <= 1.0
                => "live risk margin ratios must be in (0, 1]",
            self.risk_soft_margin_ratio <= self.risk_medium_margin_ratio
                && self.risk_medium_margin_ratio <= self.risk_hard_margin_ratio
                => "live risk margin ratios must satisfy soft <= medium <= hard",
            self.risk_soft_open_cooldown_multiplier != 0 => "LIVE_RISK_SOFT_OPEN_COOLDOWN_MULTIPLIER cannot be 0",
            self.risk_medium_reduce_positions_count != 0 => "LIVE_RISK_MEDIUM_REDUCE_POSITIONS_COUNT cannot be 0",
            self.risk_hard_close_worst_positions_count != 0 => "LIVE_RISK_HARD_CLOSE_WORST_POSITIONS_COUNT cannot be 0",
        }
    }
}

#[derive(Debug, Clone, Envconfig)]
pub struct RuntimeAlertConfig {
    #[envconfig(from = "RUNTIME_ALERT_WEBHOOK_URL", default = "")]
    pub url: String,
    #[envconfig(from = "RUNTIME_ALERT_WEBHOOK_AUTH_HEADER", default = "")]
    pub auth_header: String,
    #[envconfig(from = "RUNTIME_ALERT_WEBHOOK_TIMEOUT_SECS", default = "5")]
    pub timeout_secs: u64,
    #[envconfig(from = "RUNTIME_ALERT_WEBHOOK_MAX_RETRIES", default = "3")]
    pub max_retries: u64,
    #[envconfig(from = "RUNTIME_ALERT_WEBHOOK_RETRY_BACKOFF_MS", default = "500")]
    pub retry_backoff_ms: u64,
    #[envconfig(from = "RUNTIME_ALERT_WEBHOOK_SIGNING_SECRET", default = "")]
    pub signing_secret: String,
    #[envconfig(
        from = "RUNTIME_ALERT_WEBHOOK_SIGNING_HEADER",
        default = "X-QuantAura-Signature"
    )]
    pub signing_header: String,
    #[envconfig(
        from = "RUNTIME_ALERT_WEBHOOK_SIGNING_TIMESTAMP_HEADER",
        default = "X-QuantAura-Timestamp"
    )]
    pub signing_timestamp_header: String,
    #[envconfig(from = "RUNTIME_ALERT_WEBHOOK_SIGNING_MAX_AGE_SECS", default = "300")]
    pub signing_max_age_secs: u64,
}

impl RuntimeAlertConfig {
    fn validate(&self) {
        let webhook_url = self.url.trim();

        validate_all! {
            self.timeout_secs != 0 => "RUNTIME_ALERT_WEBHOOK_TIMEOUT_SECS cannot be 0",
            self.max_retries != 0 => "RUNTIME_ALERT_WEBHOOK_MAX_RETRIES cannot be 0",
            self.retry_backoff_ms != 0 => "RUNTIME_ALERT_WEBHOOK_RETRY_BACKOFF_MS cannot be 0",
            webhook_url.is_empty() || webhook_url.starts_with("http://") || webhook_url.starts_with("https://")
                => "RUNTIME_ALERT_WEBHOOK_URL must start with http:// or https://",
            !self.signing_enabled() || !self.signing_header.trim().is_empty()
                => "RUNTIME_ALERT_WEBHOOK_SIGNING_HEADER cannot be empty when signing secret is set",
            !self.signing_enabled() || !self.signing_timestamp_header.trim().is_empty()
                => "RUNTIME_ALERT_WEBHOOK_SIGNING_TIMESTAMP_HEADER cannot be empty when signing secret is set",
            !self.signing_enabled() || self.signing_max_age_secs != 0
                => "RUNTIME_ALERT_WEBHOOK_SIGNING_MAX_AGE_SECS cannot be 0 when signing secret is set",
        }
    }

    pub fn enabled(&self) -> bool {
        !self.url.trim().is_empty()
    }

    pub fn auth_header_set(&self) -> bool {
        !self.auth_header.trim().is_empty()
    }

    pub fn signing_enabled(&self) -> bool {
        !self.signing_secret.trim().is_empty()
    }

    pub fn signing_header_set(&self) -> bool {
        !self.signing_header.trim().is_empty()
    }

    pub fn signing_timestamp_header_set(&self) -> bool {
        !self.signing_timestamp_header.trim().is_empty()
    }
}
