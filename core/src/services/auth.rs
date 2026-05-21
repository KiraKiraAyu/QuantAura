use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{Algorithm, Header, Validation, decode, encode};
use uuid::Uuid;

use crate::{
    config::{AuthConfig, JwtConfig},
    contracts::auth::{Claims, CurrentUserPayload, MessagePayload, TokenPayload},
    error::{AppError, Result},
    repositories::UserRepo,
    state::UserRecord,
};

#[derive(Debug)]
pub struct AuthService {
    auth_config: AuthConfig,
    jwt_config: JwtConfig,
    token_blacklist: Arc<RwLock<HashMap<String, u64>>>,
    users: Arc<UserRepo>,
}

impl AuthService {
    pub fn new(
        auth_config: AuthConfig,
        jwt_config: JwtConfig,
        token_blacklist: Arc<RwLock<HashMap<String, u64>>>,
        repo: Arc<UserRepo>,
    ) -> Result<Self> {
        Ok(Self {
            auth_config,
            jwt_config,
            token_blacklist,
            users: repo,
        })
    }

    pub fn authenticate_token(&self, token: &str) -> Result<Claims> {
        let claims = self.validate_token(token)?;

        if self.is_token_blacklisted(token)? {
            return Err(AppError::Unauthorized("Token has been revoked".into()));
        }

        Ok(claims)
    }

    fn blacklist_token(&self, token: &str, exp_unix_ts: u64) -> Result<()> {
        if token.trim().is_empty() {
            return Ok(());
        }

        let mut blacklist = self
            .token_blacklist
            .write()
            .map_err(|_| AppError::Internal("Failed to lock token blacklist".into()))?;
        blacklist.insert(token.to_string(), exp_unix_ts);

        if blacklist.len() > self.auth_config.token_blacklist_max_entries {
            let now = now_unix_ts();
            blacklist.retain(|_, exp| *exp > now);
        }

        Ok(())
    }

    pub(crate) fn is_token_blacklisted(&self, token: &str) -> Result<bool> {
        if token.trim().is_empty() {
            return Ok(false);
        }

        let mut blacklist = self
            .token_blacklist
            .write()
            .map_err(|_| AppError::Internal("Failed to lock token blacklist".into()))?;

        match blacklist.get(token).copied() {
            Some(exp) if exp > now_unix_ts() => Ok(true),
            Some(_) => {
                blacklist.remove(token);
                Ok(false)
            }
            None => Ok(false),
        }
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<TokenPayload> {
        let email = normalize_email(email);
        let password = password.trim();

        if !is_valid_email(&email) || password.len() < 8 {
            return Err(AppError::BadRequest("Invalid request parameters".into()));
        }

        if self
            .users
            .find_by_email(&email)
            .await
            .map_err(|err| AppError::Internal(format!("Failed to load user: {err}")))?
            .is_some()
        {
            return Err(AppError::Conflict("Email already registered".into()));
        }

        if !self.auth_config.registration_enabled {
            return Err(AppError::Forbidden("Registration is disabled".into()));
        }

        if self.auth_config.max_users > 0
            && self
                .users
                .count()
                .await
                .map_err(|err| AppError::Internal(format!("Failed to count users: {err}")))?
                >= self.auth_config.max_users
        {
            return Err(AppError::Forbidden("Not on whitelist".into()));
        }

        let password_hash = hash_password(password)
            .map_err(|err| AppError::Internal(format!("Failed to create user: {err}")))?;

        let user = self
            .users
            .create(&email, &password_hash)
            .await
            .map_err(map_register_error)?;

        self.issue_token(&user, "Registration successful")
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<TokenPayload> {
        let email = normalize_email(email);
        let password = password.trim();

        if !is_valid_email(&email) || password.is_empty() {
            return Err(AppError::BadRequest("Invalid request parameters".into()));
        }

        let user = self
            .users
            .find_by_email(&email)
            .await
            .map_err(|err| AppError::Internal(format!("Failed to load user: {err}")))?
            .ok_or_else(|| AppError::Unauthorized("Email or password incorrect".into()))?;

        match verify_password(password, &user.password_hash) {
            Ok(true) => self.issue_token(&user, "Login successful"),
            _ => Err(AppError::Unauthorized("Email or password incorrect".into())),
        }
    }

    pub fn logout(&self, token: &str) -> Result<MessagePayload> {
        let claims = self.validate_token(token)?;

        if self.is_token_blacklisted(token)? {
            return Err(AppError::Unauthorized("Token has been revoked".into()));
        }

        let expires_at = claims.exp;

        self.blacklist_token(token, expires_at)
            .map_err(|err| AppError::Internal(format!("Failed to blacklist token: {err}")))?;

        Ok(MessagePayload {
            message: "Logged out",
        })
    }

    pub async fn current_user(&self, user_id: &str) -> Result<CurrentUserPayload> {
        let current_user = self.load_user_by_id(user_id).await?;

        Ok(CurrentUserPayload {
            user_id: current_user.id,
            email: current_user.email,
        })
    }

    pub async fn change_password(
        &self,
        user_id: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<MessagePayload> {
        let current_password = current_password.trim();
        let new_password = new_password.trim();

        if current_password.is_empty() || new_password.len() < 8 {
            return Err(AppError::BadRequest("Invalid request parameters".into()));
        }

        let user = self.load_user_by_id(user_id).await?;

        match verify_password(current_password, &user.password_hash) {
            Ok(true) => {}
            _ => return Err(AppError::Unauthorized("Current password incorrect".into())),
        }

        let password_hash = hash_password(new_password)
            .map_err(|err| AppError::Internal(format!("Failed to update password: {err}")))?;

        self.users
            .update_password_hash(&user.id, &password_hash)
            .await
            .map_err(|err| AppError::Internal(format!("Failed to update password: {err}")))?;

        Ok(MessagePayload {
            message: "Password updated",
        })
    }

    fn issue_token(&self, user: &UserRecord, message: &'static str) -> Result<TokenPayload> {
        let token = self.generate_token(&user.id)?;

        Ok(TokenPayload {
            token,
            user_id: user.id.clone(),
            email: user.email.clone(),
            message,
        })
    }

    async fn load_user_by_id(&self, user_id: &str) -> Result<UserRecord> {
        self.users
            .find_by_id(user_id)
            .await
            .map_err(|err| AppError::Internal(format!("Failed to load user: {err}")))?
            .ok_or_else(|| AppError::NotFound("User not found".into()))
    }

    fn generate_token(&self, user_id: &str) -> Result<String> {
        let issued_at = now_unix_ts();
        let expires_at = issued_at.saturating_add(self.jwt_config.ttl_secs);
        let claims = Claims {
            sub: user_id.to_string(),
            iss: self.jwt_config.issuer.clone(),
            iat: issued_at,
            exp: expires_at,
            jti: Uuid::now_v7().to_string(),
        };

        encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &self.jwt_config.encoding_key,
        )
        .map_err(|err| AppError::Internal(format!("Failed to generate token: {err}")))
    }

    fn validate_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[self.jwt_config.issuer.as_str()]);

        Ok(decode::<Claims>(token, &self.jwt_config.decoding_key, &validation)?.claims)
    }
}

fn map_register_error(err: AppError) -> AppError {
    match err {
        AppError::Conflict(_) | AppError::BadRequest(_) => err,
        err => AppError::Internal(format!("Failed to create user: {err}")),
    }
}

fn now_unix_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn hash_password(password: &str) -> bcrypt::BcryptResult<String> {
    hash(password, DEFAULT_COST)
}

fn verify_password(password: &str, password_hash: &str) -> bcrypt::BcryptResult<bool> {
    verify(password, password_hash)
}

fn is_valid_email(email: &str) -> bool {
    let has_at = email.contains('@');
    let has_dot = email
        .rsplit('@')
        .next()
        .map(|v| v.contains('.'))
        .unwrap_or(false);
    has_at && has_dot && email.len() >= 5
}

fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}
