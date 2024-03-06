use poise::serenity_prelude as serenity;
use crate::utils::base::{ Context, Error };

#[poise::command(slash_command, prefix_command)]
pub async fn check(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}