use crate::{Context, Error};
use serenity::all::EditChannel;

/// Set the slowmode for a channel.
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn slowmode(ctx: Context<'_>, seconds: u64) -> Result<(), Error> {
    let channel_id = ctx.channel_id();
    let builder = EditChannel::new().rate_limit_per_user(seconds.try_into().unwrap());

    match channel_id.edit(&ctx.http(), builder).await {
        Ok(_) => {
            ctx.say(format!("Slowmode set to {} seconds", seconds))
                .await?;
        }
        Err(e) => {
            println!(
                "Failed to set slowmode in Guild: {} Channel: {}",
                ctx.guild_id().unwrap_or_default(),
                channel_id
            );
            println!("Error: {}", e);
            ctx.say("Failed to set slowmode").await?;
        }
    };

    Ok(())
}

pub fn setup() -> Vec<poise::Command<crate::Data, Error>> {
    vec![slowmode()]
}
