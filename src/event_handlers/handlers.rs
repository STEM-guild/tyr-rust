use poise::serenity_prelude::{self, CacheHttp, ChannelId, CreateMessage};

pub async fn on_reaction_add(
    ctx: &serenity_prelude::Context,
    data: serenity_prelude::Reaction,
    channel: ChannelId, // channel to log to
) {
    let reaction_log: ChannelId = channel;
    let member = <Option<poise::serenity_prelude::Member> as Clone>::clone(
        &(&data.member), // no idea what this line does ive been doing rust for like 1 day now save me
    )
    .expect("Could not find the user from the reaction.");
    let reaction = data;
    reaction_log
        .send_message(
            ctx,
            CreateMessage::default().content(format!(
                "User {} ({}:{}) reacted with {} at {}", // carets are for disabling the embed
                member.display_name(),
                member.user.name,
                member.user.id,
                reaction.emoji.to_string(),
                reaction.message(ctx.http()).await.map_or(
                    "Could not find message (maybe it was deleted?)".to_string(),
                    |x| x.link()
                )
            )),
        )
        .await
        .expect("Could not send message to the channel. Something is going to explode.");
}

pub async fn on_reaction_remove(
    ctx: &serenity_prelude::Context,
    data: serenity_prelude::Reaction,
    channel: ChannelId, // channel to log to
) {
    channel
        .send_message(
            ctx,
            CreateMessage::default().content(format!(
                "Reaction {} removed at {}", // carets are for disabling the embed
                data.emoji.to_string(),
                data.message(ctx.http()).await.map_or(
                    "Could not find message (maybe it was deleted?)".to_string(),
                    |x| x.link()
                )
            )),
        )
        .await
        .expect("Could not send message to the channel. Something is going to explode.");
}
