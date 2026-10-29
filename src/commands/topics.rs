use crate::{Context, Error};

#[poise::command(slash_command, description_localized("en-US", "List available topics and current subscriptions"))]
pub async fn topics(ctx: Context<'_>) -> Result<(), Error> {
    let channel_id = ctx.channel_id().to_string();
    let subscribed = crate::db::get_topics_for_channel(&ctx.data().db, &channel_id).await?;

    let lines: Vec<String> = ctx.data().topics.iter().map(|t| {
        if subscribed.contains(t) {
            format!("+ {}", t)
        } else {
            format!("  {}", t)
        }
    }).collect();

    ctx.say(format!("**Available topics** (`+` = subscribed):\n```\n{}\n```", lines.join("\n"))).await?;
    Ok(())
}
