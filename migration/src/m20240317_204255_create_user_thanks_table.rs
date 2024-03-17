use sea_orm_migration::prelude::*;
use crate::sea_orm::DatabaseBackend::Sqlite;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserThank::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserThank::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserThank::UserId).integer().not_null())
                    .col(ColumnDef::new(UserThank::ThankTargetId).integer().not_null())
                    .col(ColumnDef::new(UserThank::ChannelId).integer())
                    .col(ColumnDef::new(UserThank::ThreadId).integer())
                    .col(ColumnDef::new(UserThank::MessageId).integer())
                    .col(ColumnDef::new(UserThank::CreatedAt).timestamp().default("CURRENT_TIMESTAMP"))
                    .col(ColumnDef::new(UserThank::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await?;
        if (manager.get_database_backend() != Sqlite) {
            let _ = manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_thank_user_id")
                    .from(UserThank::Table, UserThank::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            ).await?;
            let _ = manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_thank_target_id")
                    .from(UserThank::Table, UserThank::ThankTargetId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            ).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if (manager.get_database_backend() != Sqlite) {
            manager.drop_foreign_key(ForeignKey::drop().name("fk_user_thank_user_id").to_owned())
                .await?;

            manager.drop_foreign_key(ForeignKey::drop().name("fk_user_thank_target_id").to_owned())
                .await?;
        }
        manager
            .drop_table(Table::drop().table(UserThank::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    GuildId,
    DiscordId,
    Username,
    JoinedAt
}

#[derive(DeriveIden)]
enum UserThank {
    Table,
    Id,
    UserId,
    ThankTargetId,
    ChannelId,
    ThreadId,
    MessageId,
    CreatedAt,
    DeletedAt,
}
