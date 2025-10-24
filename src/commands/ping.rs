use crate::{Context, Error};

/// Show the bot's latency.
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await.as_millis();
    ctx.say(format!("Pong! {}ms", latency)).await?;
    Ok(())
}

pub fn setup() -> Vec<poise::Command<crate::Data, Error>> {
    vec![ping()]
}
