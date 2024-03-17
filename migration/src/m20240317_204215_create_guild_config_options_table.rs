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
                    .table(GuildConfigOption::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GuildConfigOption::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GuildConfigOption::GuildId)
                            .integer()
                            .not_null()
                    )
                    .col(ColumnDef::new(GuildConfigOption::OptionName).string().not_null())
                    .col(ColumnDef::new(GuildConfigOption::OptionValue).string().not_null())
                    .to_owned(),
            ).await.expect("Error executing query");

        if (manager.get_database_backend() != Sqlite) {
            manager.create_foreign_key(
                ForeignKey::create()
                    .name("fk_guild_config_option_guild_id")
                    .from(GuildConfigOption::Table, GuildConfigOption::GuildId)
                    .to(Guild::Table, Guild::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            ).await;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if (manager.get_database_backend() != Sqlite) {
            manager.drop_foreign_key(ForeignKey::drop().name("fk_guild_config_option_guild_id").to_owned())
                .await?;
        }
        manager
            .drop_table(Table::drop().table(GuildConfigOption::Table).to_owned())
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
enum GuildConfigOption {
    Table,
    Id,
    GuildId,
    OptionName,
    OptionValue,
}
