use crate::{Context, Error};

#[poise::command(slash_command, description_localized("en-US", "Check if the bot is alive"))]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await.as_millis();
    ctx.say(format!("pong - {}ms", latency)).await?;
    Ok(())
}
