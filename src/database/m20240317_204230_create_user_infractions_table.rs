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
                    .table(UserInfraction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInfraction::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserInfraction::UserId).big_integer().not_null())
                    .col(ColumnDef::new(UserInfraction::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(UserInfraction::DeletedAt).timestamp())
                    .col(ColumnDef::new(UserInfraction::InfractionType).big_integer().not_null())
                    .col(ColumnDef::new(UserInfraction::InfractionCategory).big_integer().not_null())
                    .col(ColumnDef::new(UserInfraction::InfractionNotes).string().not_null())
                    .to_owned(),
            )
            .await?;

        if (manager.get_database_backend() != Sqlite) {
            let _ = manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_user_infraction_user_id")
                        .from(UserInfraction::Table, UserInfraction::UserId)
                        .to(User::Table, User::Id)
                        // The placement for .on_delete and .on_update before to_owned was incorrect in your original code.
                        // They should be applied to the ForeignKey itself before converting to owned.
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if (manager.get_database_backend() != Sqlite) {
            manager
                .drop_foreign_key(
                    ForeignKey::drop()
                        .name("fk_user_infraction_user_id")
                        .table(UserInfraction::Table)
                        .to_owned(),
                )
                .await?;
        }
        manager
            .drop_table(
                Table::drop()
                    .table(UserInfraction::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    GuildId,
    DiscordId,
    Username,
    JoinedAt,
}

#[derive(DeriveIden)]
enum UserInfraction {
    Table,
    Id,
    UserId,
    CreatedAt, // timestamp
    DeletedAt, // timestamp
    InfractionType, // kick, ban, mute, etc
    InfractionCategory, // spam, harassment, etc
    InfractionNotes, // notes about the infraction
}