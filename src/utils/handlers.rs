use crate::utils::base::{
    Data,
    Error
};
use poise::serenity_prelude::{self as serenity, ChannelId, GuildId, MessageId};
use std::str::FromStr;


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

pub async fn on_message_delete(ctx: &serenity::Context, channel_id: &ChannelId, message_id: &MessageId, guild_id: &GuildId) {
    
    dotenv::dotenv().expect("Failed to load .env file");
    let message_log_channel_id = dotenv::var("MESSAGE_LOG_CHANNEL_ID").expect("Expected a message_log_channel_id in the environment");
    let message_log_channel = serenity::ChannelId::from_str(&message_log_channel_id).expect("Invalid message_log_channel_id");
            
    match serenity::cache::Cache::message(&ctx.cache, channel_id, message_id) {
        Some(m) => {
            match ctx.cache.member(guild_id, m.author.id) {
                Some(cached_author) => {
                    let say = format!("Message deleted. The message was sent by {} ({}, {}) in <#{}>:",
                        cached_author.nick.clone().unwrap_or_else(|| m.author.global_name.clone().unwrap_or_else(|| String::from(""))), //chooses the first one that exists: nickname -> displayname -> ""
                        m.author.name,
                        m.author.id.to_string(),
                        m.channel_id,
                    );
                    match message_log_channel.say(&ctx.http, say).await {
                        Err(why) => {
                            println!("Failed to send message: {:?}", why);
                        }
                        Ok(log_message) => {
                            if m.content.chars().count() <= 2000 {
                                if let Err(why) = log_message.reply(&ctx.http, m.content.clone()).await{
                                    println!("Failed to send message: {:?}", why);
                                }
                            } else {
                                // TODO: should send as file. this will be triggered if a nitro user sends a long message as they have a limit of 4000 chars.
                                println!("someone deleted a very long message");
                            }
                            
                        }
                    }
                }
                None => {
                    panic!("Failed to get message from cache :(");
                }
            }
        }
        None => {
            panic!("Failed to get message from cache :(");
        }
    }
}