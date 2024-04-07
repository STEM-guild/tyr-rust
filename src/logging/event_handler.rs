use std::str::FromStr;
use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage};
use crate::utils::base::Data;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data
) {
    match event {
        serenity::FullEvent::Ready { .. } => {
            serenity::cache::Cache::set_max_messages(&ctx.cache, usize::MAX)
        }
        serenity::FullEvent::MessageUpdate {
            old_if_available,
            new,
            event
        } => {
            let author = event.author.as_ref().expect("Author missing from MessageUpdate event");
            let old_message = old_if_available.as_ref();
            let new_message = new.as_ref();

            let embed = CreateEmbed::new()
                .author(CreateEmbedAuthor::new(format!("{} ({})", ({
                    match author.discriminator {
                        None => {author.name.clone()}
                        Some(discriminator) => {format!("{}#{}", &author.name, discriminator)}
                    }
                }), author.id)).icon_url(author.avatar_url().unwrap_or_else(|| "".to_string())))
                .title(if new_message.is_some() { "Link to message" } else { "" })
                .url({
                    if let Some(new_message) = new_message {
                        new_message.link()
                    } else { "".to_string() }
                })
                .footer(CreateEmbedFooter::new(format!("Message {} in channel {}", event.id, event.channel_id))
                )
                .field("Original message", if old_message.is_none() {"No message content available"} else {&old_message.unwrap().content}, false)
                .field("Edited Message", if new_message.is_none() {"No message content available"} else {&new_message.unwrap().content}, false);
            
            serenity::ChannelId::from_str(dotenv!("MESSAGE_EDIT")).expect("Unable to find Message Edit log channel by id").send_message(&ctx.http, CreateMessage::new().embed(embed)).await.ok();
        }
        _ => {}
    }
}