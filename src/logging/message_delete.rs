use std::str::FromStr;
use std::time::Duration;
use dotenv_codegen::dotenv;
use poise::{serenity_prelude as serenity};
use poise::serenity_prelude::{ChannelId, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, GuildId, MessageAction, MessageId, Timestamp, UserId};
use poise::serenity_prelude::model::guild::audit_log::Action as AuditLogAction;
use crate::utils::helpers;

pub async fn log_message_delete(ctx: &serenity::Context, channel_id: &ChannelId, message_id: &MessageId, guild_id: &Option<GuildId>) {

    let message_author_id = ctx.cache.message(channel_id, message_id).map(|message| message.author.id);

    let deleter_user_id: Option<UserId> = if let Some(future) = message_author_id.zip(guild_id.as_ref()).map(|(user_id, guild_id)| lookup_deleter_in_audit_log(ctx, guild_id, user_id)) {
        future.await
    } else { None };

    if guild_id.is_some() {
        tokio::time::sleep(Duration::from_secs(1)).await; // Give audit log a moment to work (yes this is a race condition)
    }
    let embed = if let Some(message) = ctx.cache.message(channel_id, message_id) {
        CreateEmbed::new()
            .author(CreateEmbedAuthor::new(format!("{} ({})", helpers::format_user(&message.author), &message.author.id)).url(&message.author.avatar_url().unwrap_or_else(|| "".to_string())))
            .field("Deleted message", &message.content, false)
            .field("Likely deleted by", {
                if let Some(deleter_user_id) = deleter_user_id {
                    helpers::format_user_id(deleter_user_id)
                } else {
                    "Themselves".to_string()
                }
            }, false)
            .footer(CreateEmbedFooter::new(format!("{}", message.author.id)))
    } else {
        CreateEmbed::new()
            .footer(CreateEmbedFooter::new(format!("Message {} in {}", message_id, channel_id)))
    };

    ChannelId::from_str(dotenv!("MESSAGE_DELETE")).expect("Unable to find Message Delete log channel by id").send_message(&ctx.http, CreateMessage::new().embed(embed)).await.ok();
}

async fn lookup_deleter_in_audit_log(ctx: &serenity::Context, guild_id: &GuildId, deleted_user_id: UserId) -> Option<UserId> {
    let audit_logs = guild_id.audit_logs(&ctx.http, Some(AuditLogAction::Message(MessageAction::Delete)), None, None, Some(16u8)).await.ok()?;

    audit_logs.entries.iter().filter(|entry| {
        (Timestamp::now().timestamp() - entry.id.created_at().timestamp()) < 10
    }).find(|entry| entry.target_id.unwrap_or_default().get() == deleted_user_id.get()).map(|entry| entry.user_id)
}

/*async fn lookup_deleter_in_audit_log(ctx: &serenity::Context, guild_id: &GuildId, deleted_user_id: UserId) -> Option<UserId> {
    if let Ok(audit_logs) = guild_id.audit_logs(&ctx.http, Some(AuditLogAction::Message(MessageAction::Delete)), None, None, Some(16u8)).await {
        if let Some(entry) = audit_logs.entries.iter().filter(|entry| {
            (Timestamp::now().timestamp() - entry.id.created_at().timestamp()) < 10
        }).find(|entry| {
            if let Some(target_id) = entry.target_id {
                target_id.get() == deleted_user_id.get()
            } else { false }
        }) {
            Some(entry.user_id)
        } else { None }
    } else { None }
}*/