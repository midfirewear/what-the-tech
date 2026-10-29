use crate::{Context, Error};

#[poise::command(slash_command, description_localized("en-US", "Subscribe this channel to a topic"))]
pub async fn subscribe(
    ctx: Context<'_>,
    #[description = "Topic to subscribe to"] topic: String,
) -> Result<(), Error> {
    let topic = topic.to_lowercase();

    if !ctx.data().topics.contains(&topic) {
        let available = ctx.data().topics.join(", ");
        ctx.say(format!("Unknown topic `{}`. Available: {}", topic, available)).await?;
        return Ok(());
    }

    let channel_id = ctx.channel_id().to_string();
    let user_id = ctx.author().id.to_string();

    crate::db::add_subscription(&ctx.data().db, &channel_id, &topic, &user_id).await?;
    ctx.say(format!("Subscribed this channel to `{}`.", topic)).await?;
    Ok(())
}
