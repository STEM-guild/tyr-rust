use std::str::FromStr;
use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{ChannelId, CreateEmbed, CreateEmbedFooter, CreateMessage, MessageId, Reaction, Timestamp};
use crate::utils::helpers;

pub async fn reaction_create(ctx: &serenity::Context, add_reaction: &Reaction){
    let embed = CreateEmbed::new()
        .title("Link to message")
        .url(add_reaction.message_id.link(add_reaction.channel_id, add_reaction.guild_id))
        .field("Reaction added", format!("{}", add_reaction.emoji), true)
        .field("Author", if let Some(user_id) = add_reaction.user_id {
            helpers::format_user_id(user_id)
        } else { "Unknown".to_string() }, true)
        .footer(CreateEmbedFooter::new(format!("Message {} in {}", add_reaction.message_id, add_reaction.channel_id)))
        .timestamp(Timestamp::now());

        ChannelId::from_str(dotenv!("REACTION")).expect("Unable to find Reaction log channel by id").send_message(&ctx.http, CreateMessage::new().embed(embed)).await.ok();
}

pub async fn reaction_remove(ctx: &serenity::Context, removed_reaction: &Reaction){
    let embed = CreateEmbed::new()
        .title("Link to message")
        .url(removed_reaction.message_id.link(removed_reaction.channel_id, removed_reaction.guild_id))
        .field("Reaction removed", format!("{}", removed_reaction.emoji), true)
        .field("Author", if let Some(user_id) = removed_reaction.user_id {
            helpers::format_user_id(user_id)
        } else { "Unknown".to_string() }, true)
        .footer(CreateEmbedFooter::new(format!("Message {} in {}", removed_reaction.message_id, removed_reaction.channel_id)))
        .timestamp(Timestamp::now());

    ChannelId::from_str(dotenv!("REACTION")).expect("Unable to find Reaction log channel by id").send_message(&ctx.http, CreateMessage::new().embed(embed)).await.ok();
}

pub async fn reaction_remove_all(ctx: &serenity::Context, channel_id: &ChannelId, removed_from_message_id: &MessageId){
    let guild_id = if let Some(channel) = ctx.cache.channel(channel_id) {
        Some(channel.guild_id)
    } else if let Some(message) = ctx.cache.message(channel_id, removed_from_message_id) {
        message.guild_id
    } else { None };

    let embed = CreateEmbed::new()
        .title("Link to message")
        .url(removed_from_message_id.link_ensured(&ctx.http, channel_id.clone(), guild_id).await)
        .description("**All reactions removed**")
        .footer(CreateEmbedFooter::new(format!("Message {} in {}", removed_from_message_id, channel_id)))
        .timestamp(Timestamp::now());

    ChannelId::from_str(dotenv!("REACTION")).expect("Unable to find Reaction log channel by id").send_message(&ctx.http, CreateMessage::new().embed(embed)).await.ok();
}

pub async fn reaction_remove_emoji(ctx: &serenity::Context, removed_reactions: &Reaction){
    let embed = CreateEmbed::new()
        .title("Link to message")
        .url(removed_reactions.message_id.link(removed_reactions.channel_id, removed_reactions.guild_id))
        .field("Reaction purged from message", format!("{}", removed_reactions.emoji), false)
        .footer(CreateEmbedFooter::new(format!("Message {} in {}", removed_reactions.message_id, removed_reactions.channel_id)))
        .timestamp(Timestamp::now());

    ChannelId::from_str(dotenv!("REACTION")).expect("Unable to find Reaction log channel by id").send_message(&ctx.http, CreateMessage::new().embed(embed)).await.ok();
}