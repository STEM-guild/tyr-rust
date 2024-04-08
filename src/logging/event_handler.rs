use poise::serenity_prelude as serenity;
use crate::logging::message_delete::log_message_delete;
use crate::logging::message_edit::log_message_edit;
use crate::utils::base::{Data, Error};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data
) {
    match event {
        serenity::FullEvent::Ready { .. } => {
            serenity::cache::Cache::set_max_messages(&ctx.cache, usize::MAX) // Ticking time bomb
        }
        serenity::FullEvent::MessageUpdate {
            old_if_available,
            new,
            event
        } => { log_message_edit(ctx, old_if_available, new, event).await    ; }
        serenity::FullEvent::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id
        } => { log_message_delete(ctx, channel_id, deleted_message_id, guild_id).await; }
        _ => {}
    }
}