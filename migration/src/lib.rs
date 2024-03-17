pub use sea_orm_migration::prelude::*;

mod m20240317_204202_create_guilds_table;
mod m20240317_204215_create_guild_config_options_table;
mod m20240317_204225_create_users_table;
mod m20240317_204230_create_user_infractions_table;
mod m20240317_204255_create_user_thanks_table;
// mod m20240317_204305_create_mod_actions_table;
// mod m20240317_204309_create_messages_table;
// mod m20240317_204313_create_reactions_table;
// mod m20240317_205009_create_user_name_changes_table;
// mod m20240317_205921_create_user_staff_notes_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240317_204202_create_guilds_table::Migration),
            Box::new(m20240317_204215_create_guild_config_options_table::Migration),
            Box::new(m20240317_204225_create_users_table::Migration),
            Box::new(m20240317_204230_create_user_infractions_table::Migration),
            Box::new(m20240317_204255_create_user_thanks_table::Migration),
            // Box::new(m20240317_204305_create_mod_actions_table::Migration),
            // Box::new(m20240317_204309_create_messages_table::Migration),
            // Box::new(m20240317_204313_create_reactions_table::Migration),
            // Box::new(m20240317_205009_create_user_name_changes_table::Migration),
            // Box::new(m20240317_205921_create_user_staff_notes_table::Migration),
        ]
    }
}
