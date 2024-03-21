use crate::utils::base::{
    Data,
    Error
};
use poise::serenity_prelude::{self as serenity, ChannelId, GuildId, MessageId};

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

pub fn on_message_delete(ctx: &serenity::Context, channel_id: &ChannelId, message_id: &MessageId, guild_id: &GuildId) -> Result<[String; 2],&'static str>{
    match serenity::cache::Cache::message(&ctx.cache, channel_id, message_id) {
        Some(m) => {
            match ctx.cache.member(guild_id, m.author.id) {
                Some(cached_author) => {
                    Ok([format!("Message deleted. The message was sent by {} ({}, {}) in <#{}>:",
                        cached_author.nick.clone().unwrap_or_else(|| m.author.global_name.clone().unwrap_or_else(|| String::from(""))), //chooses the first one that exists: nickname -> displayname -> ""
                        m.author.name,
                        m.author.id.to_string(),
                        m.channel_id,
                    ),
                    format!("\"{}\"", m.content.clone())])
                }
                None => {
                    Ok([format!("Message deleted. The message was sent by {} ({}, {}) in <#{}>:",
                        m.author.global_name.clone().unwrap_or_else(|| String::from("")),
                        m.author.name,
                        m.author.id.to_string(),
                        m.channel_id,
                    ), m.content.clone()])
                }
            }
        }
        None => {
            Err("Failed to get message from cache :(")
        }
    }
}