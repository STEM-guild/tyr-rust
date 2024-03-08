#![warn(clippy::str_to_string)]

mod utils;
mod commands;

use poise::serenity_prelude as serenity;
use std::{
    collections::HashMap, str::FromStr, sync::{Arc, Mutex}, time::Duration
};

use utils::{
    base::{Data,Error},
    handlers::{self, on_error},
};

#[tokio::main]
async fn main() {
    env_logger::init();

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::messages::help(), 
            commands::messages::vote(), 
            commands::messages::getvotes(), 
            commands::messages::poll(),
            commands::dev::ping(),
            commands::user::check(),
            commands::messages::jask(),
            commands::messages::ask(),
            commands::dev::register(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(3600),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("hey bot"),
                poise::Prefix::Literal("hey bot,"),
            ],
            ..Default::default()
        },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                //    return Ok(false);
                }
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    dotenv::dotenv().expect("Failed to load .env file");
    let guild_id = dotenv::var("GUILD_ID").expect("Expected a guild_id in the environment");

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_in_guild(ctx, &framework.options().commands, serenity::GuildId::from_str(&guild_id).expect("invalid guild ID")).await?;
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                })
            })
        })
        .options(options)
        .build();

    dotenv::dotenv().expect("Failed to load .env file");
    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILD_PRESENCES;
        //                                                                                                ↑ for member caching ↑
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            serenity::cache::Cache::set_max_messages(&ctx.cache, usize::MAX); // for running on lower end hardware, if need be, this should be reduced. cache clears every hour by default.
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::MessageDelete { channel_id, deleted_message_id, guild_id } => {
            
            match guild_id {
                Some(guild_id) => {
                    handlers::on_message_delete(ctx, channel_id, deleted_message_id, guild_id).await;
                },
                None => () // DM, so we can exit
            }    
        }
        _ => {}
    }
    Ok(())
}