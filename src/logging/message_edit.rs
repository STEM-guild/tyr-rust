use std::str::FromStr;
use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Message, MessageUpdateEvent, CreateMessage, Timestamp};
use crate::utils::helpers;

pub async fn log_message_edit(ctx: &serenity::Context, old_if_available: &Option<Message>, new: &Option<Message>, event: &MessageUpdateEvent){
    let author = event.author.as_ref().expect("Author missing from MessageUpdate event");
    let old_message = old_if_available.as_ref();
    let new_message = new.as_ref();

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(format!("{} ({})", helpers::format_username(author), author.id)).icon_url(author.avatar_url().unwrap_or_else(|| "".to_string())))
        .title(if new_message.is_some() { "Link to message" } else { "" })
        .url({
        if let Some(new_message) = new_message {
        new_message.link()
        } else { "".to_string() }
        })
        .footer(CreateEmbedFooter::new(format!("Message {} in channel {}", event.id, event.channel_id))
        )
        .field("Original message", if let Some(old_message) = old_message {&old_message.content} else {"No message content available"}, false)
        .field("Edited Message",
               if let Some(new_message) = new_message {
                        new_message.content.clone()
                    } else if let Ok(new_message) = ctx.http.get_message(event.channel_id, event.id).await {
                        new_message.content
                    } else {
                        "No message content available".to_string()
                    }, false)
        .timestamp(Timestamp::now());

    serenity::ChannelId::from_str(dotenv!("MESSAGE_EDIT")).expect("Unable to find Message Edit log channel by id").send_message(&ctx.http, CreateMessage::new().embed(embed)).await.ok();
}