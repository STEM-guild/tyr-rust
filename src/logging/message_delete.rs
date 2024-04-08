use std::str::FromStr;
use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{ChannelId, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, MessageId, Timestamp};
use crate::utils::helpers;

pub async fn log_message_delete(ctx: &serenity::Context, channel_id: &ChannelId, message_id: &MessageId) {
    let embed = if let Some(message) = ctx.cache.message(channel_id, message_id) {
        CreateEmbed::new()
            .author(CreateEmbedAuthor::new(format!("{} ({})", helpers::format_username(&message.author), &message.author.id)).url(&message.author.avatar_url().unwrap_or_else(|| "".to_string())))
            .field("Deleted message", &message.content, false)
            .footer(CreateEmbedFooter::new(format!("Message {} in {}", message_id, channel_id)))
            .timestamp(Timestamp::now())
    } else {
        CreateEmbed::new()
            .footer(CreateEmbedFooter::new(format!("Message {} in {}", message_id, channel_id)))
            .timestamp(Timestamp::now())
    };

    ChannelId::from_str(dotenv!("MESSAGE_DELETE")).expect("Unable to find Message Delete log channel by id").send_message(&ctx.http, CreateMessage::new().embed(embed)).await.ok();
}