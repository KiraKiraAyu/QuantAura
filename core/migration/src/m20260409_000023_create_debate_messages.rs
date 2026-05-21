use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DebateMessages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DebateMessages::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DebateMessages::DebateId).string().not_null())
                    .col(
                        ColumnDef::new(DebateMessages::Round)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(DebateMessages::Personality)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(DebateMessages::Role)
                            .string()
                            .not_null()
                            .default("assistant"),
                    )
                    .col(
                        ColumnDef::new(DebateMessages::Content)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(DebateMessages::Vote)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(DebateMessages::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_debate_messages_debate_round")
                    .table(DebateMessages::Table)
                    .col(DebateMessages::DebateId)
                    .col((DebateMessages::Round, IndexOrder::Asc))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(DebateMessages::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum DebateMessages {
    Table,
    Id,
    DebateId,
    Round,
    Personality,
    Role,
    Content,
    Vote,
    CreatedAt,
}
