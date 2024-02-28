use std::str::FromStr;

use poise::serenity_prelude::{self, futures::TryFutureExt, ReactionType};

use crate::{Context, Error};

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

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ping(_ctx: Context<'_>) -> Result<(), Error> {
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
            if let Some(reaction) = serenity_prelude::parse_emoji(potential) {
                x.react(ctx.http(), reaction).await?;
            } else {
                x.react(ctx.http(), ReactionType::from_str(potential).unwrap()).await?;
            }
        }
        Ok(())
    }).await?;
    Ok(())
}
