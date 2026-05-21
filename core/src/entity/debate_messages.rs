//! `SeaORM` Entity, hand-maintained for Debate Arena.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "debate_messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text")]
    pub debate_id: String,
    pub round: i32,
    #[sea_orm(column_type = "Text")]
    pub personality: String,
    #[sea_orm(column_type = "Text")]
    pub role: String,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    #[sea_orm(column_type = "Text")]
    pub vote: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
