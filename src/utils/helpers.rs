use poise::serenity_prelude as serenity;
use poise::serenity_prelude::UserId;

pub fn format_user(user: &serenity::User) -> String {
    match user.discriminator {
        None => {user.name.clone()}
        Some(discriminator) => {format!("{}#{}", &user.name, discriminator)}
    }
}

pub fn format_user_id(user_id: UserId) -> String {
    format!("<@{}> ({})", user_id, user_id)
}