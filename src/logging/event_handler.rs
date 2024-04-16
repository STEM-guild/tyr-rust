use poise::serenity_prelude as serenity;
use crate::logging::message_delete::log_message_delete;
use crate::logging::message_edit::log_message_edit;
use crate::logging::reaction::{reaction_create, reaction_remove, reaction_remove_all, reaction_remove_emoji};
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
        } => { log_message_edit(ctx, old_if_available, new, event).await; }
        serenity::FullEvent::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id
        } => { log_message_delete(ctx, channel_id, deleted_message_id, guild_id).await; }

        serenity::FullEvent::ReactionAdd {
            add_reaction
        } => { reaction_create(ctx, add_reaction).await }
        serenity::FullEvent::ReactionRemove {
            removed_reaction
        } => { reaction_remove(ctx, removed_reaction).await }
        serenity::FullEvent::ReactionRemoveAll {
            channel_id,
            removed_from_message_id
        } => { reaction_remove_all(ctx, channel_id, removed_from_message_id).await }
        serenity::FullEvent::ReactionRemoveEmoji {
            removed_reactions
        } => { reaction_remove_emoji(ctx, removed_reactions).await }

        _ => {}
    }
}