use poise::serenity_prelude::{ self as serenity, futures::TryFutureExt, ReactionType };
use crate::utils::base::{ Context, Error };
use std::str::FromStr;
use std::num::ParseIntError;

#[poise::command(slash_command, prefix_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
        .await?;
    Ok(())
}

/// Vote for something
///
/// Enter `!vote pumpkin` to vote for pumpkins
#[poise::command(prefix_command, slash_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), Error> {
    // Lock the Mutex in a block {} so the Mutex isn't locked across an await point
    let num_votes = {
        let mut hash_map = ctx.data().votes.lock().unwrap();
        let num_votes = hash_map.entry(choice.clone()).or_default();
        *num_votes += 1;
        *num_votes
    };

    let response = format!("Successfully voted for {choice}. {choice} now has {num_votes} votes!");
    ctx.say(response).await?;
    Ok(())
}

/// Retrieve number of votes
///
/// Retrieve the number of votes either in general, or for a specific choice:
/// ```
/// !getvotes
/// !getvotes pumpkin
/// ```
#[poise::command(prefix_command, track_edits, aliases("votes"), slash_command)]
pub async fn getvotes(
    ctx: Context<'_>,
    #[description = "Choice to retrieve votes for"] choice: Option<String>,
) -> Result<(), Error> {
    if let Some(choice) = choice {
        let num_votes = *ctx.data().votes.lock().unwrap().get(&choice).unwrap_or(&0);
        let response = match num_votes {
            0 => format!("Nobody has voted for {} yet", choice),
            _ => format!("{} people have voted for {}", num_votes, choice),
        };
        ctx.say(response).await?;
    } else {
        let mut response = String::new();
        for (choice, num_votes) in ctx.data().votes.lock().unwrap().iter() {
            response += &format!("{}: {} votes", choice, num_votes);
        }

        if response.is_empty() {
            response += "Nobody has voted for anything yet :(";
        }

        ctx.say(response).await?;
    };

    Ok(())
}

#[poise::command(prefix_command, aliases("makepoll"))]
pub async fn poll(
    ctx: Context<'_>,
    #[description = "unused : is there a way to get a list of options?"] _i_dont_know_how_to_get_alist_of_options: Vec<String>,
) -> Result<(), Error> {
    // ctx.say(ctx.invocation_string()).await?;
    let mut potential_emotes: Vec<&str> = vec![];
    ctx.say(ctx.invocation_string().split("\n").skip(1).map(|x| {
        let trying = x.split(" ").next().unwrap();
        potential_emotes.push(trying);
        format!("{}", x)
    }).fold("".into(), |acc, e| { format!("{}{}\n", acc, e) })).await?.into_message().and_then(|x| async move {
        for potential in potential_emotes {
            if let Some(reaction) = serenity::parse_emoji(potential) {
                x.react(ctx.http(), reaction).await?;
            } else {
                x.react(ctx.http(), ReactionType::from_str(potential).unwrap()).await?;
            }
        }
        Ok(())
    }).await?;
    Ok(())
}

// posts a useful link to the dont ask to ask website.

// maybe having this in an embed would be better, we can experiment
#[poise::command(slash_command, prefix_command)]
pub async fn jask(
    ctx: Context<'_>,
    #[description = "User to ping."] user: Option<poise::serenity_prelude::User>,
) -> Result<(), Error> {
    if user.as_ref() == None {
        ctx.say("Don't ask to ask, just ask! We'll be happy to help! Read more [here](https://dontasktoask.com/).").await?;
    } else {
        let u = user.as_ref().unwrap();
        let response = format!("Hey <@{}>, next time, don't ask to ask, just ask! We'll be happy to help! Read more [here](https://dontasktoask.com/).", u.id);
        ctx.say(response).await?;
    }
    Ok(())
}

// This command helps keep #general clean by redircting new users to help channels

// a few suggestion:
// once docs are written, we should add a "how to ask question" channel and add that to the message
// maybe having this in an embed would be better, we can experiment.
// perhaps Tyr can ping the user in a help text channel if it's not too busy? or maybe create a post in forum? that will self-dstruct? many options - none desprate.
// once DB is up we should have some kind of dictionary with aliases and channel IDs so this gross match clause can be removed.
#[poise::command(slash_command, prefix_command)]
pub async fn ask(
    ctx: Context<'_>,
    #[description = "User to ping."] user: Option<poise::serenity_prelude::User>,
    #[description = "The subject to redirect to."] subjects: Option<String>,
) -> Result<(), Error> {
    let def = &String::from_str("").unwrap();
    let subject = subjects.as_ref().unwrap_or(def);
    let redirect: String;
    match subject.to_lowercase().as_str() {
        "math" | "maths" => {
            redirect = format!("<#{}> or <#{}>, ping helpers, and wait for someone to respond.", channel_id_from_key("MATHS_HELP_TEXT_ID").expect("Could not get channel ID"), channel_id_from_key("MATHS_HELP_FORUM_ID").expect("Could not get channel ID"));
        }
        "physics" => {
            redirect = format!("<#{}> or <#{}>, ping helpers, and wait for someone to respond.", channel_id_from_key("PHYSICS_HELP_TEXT_ID").expect("Could not get channel ID"), channel_id_from_key("PHYSICS_HELP_FORUM_ID").expect("Could not get channel ID"));
        }
        "bio" | "biology" => {
            redirect = format!("<#{}> or <#{}>, ping helpers, and wait for someone to respond.", channel_id_from_key("BIO_HELP_TEXT_ID").expect("Could not get channel ID"), channel_id_from_key("BIO_HELP_FORUM_ID").expect("Could not get channel ID"));
        }
        "chem" | "chemistry" => {
            redirect = format!("<#{}> or <#{}>, ping helpers, and wait for someone to respond.", channel_id_from_key("CHEM_HELP_TEXT_ID").expect("Could not get channel ID"), channel_id_from_key("CHEM_HELP_FORUM_ID").expect("Could not get channel ID"));
        }
        "engineering" => {
            redirect = format!("<#{}> or <#{}>, ping helpers, and wait for someone to respond.", channel_id_from_key("ENGINEERING_HELP_TEXT_ID").expect("Could not get channel ID"), channel_id_from_key("ENGINEERING_HELP_FORUM_ID").expect("Could not get channel ID"));
        }
        "coding" | "programming" | "cs" | "computer science" | "tech" => {
            redirect = format!("<#{}> or <#{}>, ping helpers, and wait for someone to respond.", channel_id_from_key("PROGRAMMING_HELP_TEXT_ID").expect("Could not get channel ID"), channel_id_from_key("PROGRAMMING_HELP_FORUM_ID").expect("Could not get channel ID"));
        }
        "psychology" => {
            redirect = format!("<#{}>, ping helpers, and wait for someone to respond.", channel_id_from_key("PSYCHOLOGY_HELP_TEXT_ID").expect("Could not get channel ID"));
        }
        "other" | "other help" => {
            redirect = format!("<#{}> or <#{}>, ping helpers, and wait for someone to respond.", channel_id_from_key("OTHER_HELP_TEXT_ID").expect("Could not get channel ID"), channel_id_from_key("OTHER_HELP_FORUM_ID").expect("Could not get channel ID"));
        }
        &_ => {
            redirect = format!("the help channel / forum of your choosing.")
        }
    }

    if user.as_ref() == None {
        let response = format!("Please post your question in {}", redirect);
        ctx.say(response).await?;
    } else {
        let u = user.as_ref().unwrap();
        let response = format!("Hey <@{}>, please post your question in {}", u.id, redirect);
        ctx.say(response).await?;
    }
    Ok(())
}


// NOTE: this function should not be here once database is set up!!
// will panic! not a good option long-term

#[allow(dead_code)]
fn channel_id_from_key(key: &str) -> Result<poise::serenity_prelude::model::id::ChannelId,ParseIntError>{
    let channel_id = dotenv::var(key).expect(&format!("Expected {} in the environment", key));
    match poise::serenity_prelude::model::id::ChannelId::from_str(&channel_id) {
        Err(why) => {
            Result::Err(why)
        }
        Ok(ok) => {
            Result::Ok(ok)
        }
    }
}