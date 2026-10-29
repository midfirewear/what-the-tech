use crate::{Context, Error};

#[poise::command(slash_command, description_localized("en-US", "Unsubscribe this channel from a topic"))]
pub async fn unsubscribe(
    ctx: Context<'_>,
    #[description = "Topic to unsubscribe from"] topic: String,
) -> Result<(), Error> {
    let topic = topic.to_lowercase();
    let channel_id = ctx.channel_id().to_string();

    let subscribed = crate::db::get_topics_for_channel(&ctx.data().db, &channel_id).await?;

    if !subscribed.contains(&topic) {
        ctx.say(format!("This channel is not subscribed to `{}`.", topic)).await?;
        return Ok(());
    }

    crate::db::remove_subscription(&ctx.data().db, &channel_id, &topic).await?;
    ctx.say(format!("Unsubscribed this channel from `{}`.", topic)).await?;
    Ok(())
}
