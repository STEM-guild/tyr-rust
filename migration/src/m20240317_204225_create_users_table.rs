use sea_orm_migration::prelude::*;
use crate::sea_orm::DatabaseBackend::Sqlite;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(User::GuildId).integer().not_null())
                .col(ColumnDef::new(User::DiscordId).integer().not_null())
                .col(ColumnDef::new(User::Username).string().not_null())
                .col(ColumnDef::new(User::JoinedAt).timestamp().not_null())
                .to_owned()
        ).await?;
        if (manager.get_database_backend() != Sqlite) {
            let _ = manager.create_foreign_key(ForeignKey::create()
                .name("fk_user_guild")
                .from(User::Table, User::GuildId)
                .to(Guild::Table, Guild::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
                .to_owned())
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if (manager.get_database_backend() != Sqlite) {
            manager
                .drop_foreign_key(ForeignKey::drop().name("fk_user_guild").to_owned())
                .await?;
        }
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    DiscordId,
    Name
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
