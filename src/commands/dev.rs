// these really should be exclusive to dev team roles, but we cross that bridge when it comes across us

use std::str::FromStr;
use std::time::Duration;
use dotenv_codegen::dotenv;

use poise::serenity_prelude;
use poise::serenity_prelude::CreateMessage;

use crate::utils::base::{ Context, Error };

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply(format!("Pong! {}ms", ctx.ping().await.as_millis())).await?;
    Ok(())
}

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn deleteself(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx.channel_id().send_message(ctx.http(), CreateMessage::new().content("Deleting in 5s!")).await?;
    tokio::time::sleep(Duration::from_secs(5)).await;
    msg.delete(ctx.http()).await?;
    Ok(())
}

// this command lets devs control the registration of slash-commands in
// guild and global contexts
#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    let dev_role_id = dotenv!("DEV_ROLE_ID");
    let guild_id = dotenv!("GUILD_ID");

    if ctx.author().has_role(ctx.http(), serenity_prelude::GuildId::from_str(&guild_id).expect("invalid guild ID"), serenity_prelude::RoleId::from_str(&dev_role_id).expect("invalid dev role ID")
).await? {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    } else {
        ctx.say("Missing permissions!").await?;
    }

    Ok(())
}