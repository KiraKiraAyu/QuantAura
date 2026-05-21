use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set, prelude::Expr,
};
use uuid::Uuid;

use crate::entity::users;
use crate::time::{dt_to_ts, ts_to_dt};
use crate::{
    error::{AppError, Result as AppResult},
    state::UserRecord,
};

#[derive(Debug, Clone)]
pub struct UserRepo {
    db: DatabaseConnection,
}

impl UserRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn count(&self) -> AppResult<usize> {
        users::Entity::find()
            .count(&self.db)
            .await
            .map(|count| count as usize)
            .map_err(AppError::from)
    }

    pub async fn find_by_id(&self, user_id: &str) -> AppResult<Option<UserRecord>> {
        users::Entity::find_by_id(user_id.trim().to_string())
            .one(&self.db)
            .await
            .map(|row| row.map(into_user_record))
            .map_err(AppError::from)
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<UserRecord>> {
        let normalized = normalize_email(email);

        users::Entity::find()
            .filter(users::Column::Email.eq(normalized))
            .one(&self.db)
            .await
            .map(|row| row.map(into_user_record))
            .map_err(AppError::from)
    }

    pub async fn create(&self, email: &str, password_hash: &str) -> AppResult<UserRecord> {
        let email = normalize_email(email);
        if email.is_empty() || password_hash.trim().is_empty() {
            return Err(AppError::BadRequest("Invalid credentials".into()));
        }

        let now = now_unix_ts();
        let user = UserRecord {
            id: Uuid::now_v7().to_string(),
            email,
            password_hash: password_hash.to_string(),
            created_at: now,
            updated_at: now,
        };

        users::ActiveModel {
            id: Set(user.id.clone()),
            email: Set(user.email.clone()),
            password_hash: Set(user.password_hash.clone()),
            created_at: Set(ts_to_dt(user.created_at as i64)),
            updated_at: Set(ts_to_dt(user.updated_at as i64)),
        }
        .insert(&self.db)
        .await
        .map_err(|e| {
            if e.to_string().to_ascii_lowercase().contains("unique") {
                return AppError::Conflict("Email already registered".into());
            }
            AppError::from(e)
        })?;

        Ok(user)
    }

    pub async fn update_password_hash(&self, user_id: &str, password_hash: &str) -> AppResult<()> {
        if password_hash.trim().is_empty() {
            return Err(AppError::BadRequest("Invalid credentials".into()));
        }

        let affected = users::Entity::update_many()
            .col_expr(
                users::Column::PasswordHash,
                Expr::value(password_hash.to_string()),
            )
            .col_expr(
                users::Column::UpdatedAt,
                Expr::value(ts_to_dt(now_unix_ts() as i64)),
            )
            .filter(users::Column::Id.eq(user_id.trim()))
            .exec(&self.db)
            .await?
            .rows_affected;

        if affected == 0 {
            return Err(AppError::NotFound("User not found".into()));
        }

        Ok(())
    }
}

fn into_user_record(row: users::Model) -> UserRecord {
    UserRecord {
        id: row.id,
        email: row.email,
        password_hash: row.password_hash,
        created_at: dt_to_ts(row.created_at).max(0) as u64,
        updated_at: dt_to_ts(row.updated_at).max(0) as u64,
    }
}

fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

fn now_unix_ts() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
