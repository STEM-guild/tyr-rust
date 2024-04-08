use poise::serenity_prelude as serenity;

pub fn format_username(user: &serenity::User) -> String {
    match user.discriminator {
        None => {user.name.clone()}
        Some(discriminator) => {format!("{}#{}", &user.name, discriminator)}
    }
}