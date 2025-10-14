use crate::{Context, Error};

#[poise::command(slash_command, guild_only)]
pub async fn xlinkconvert(
    ctx: Context<'_>,
    #[description = "変換するURL"]
    url: String,
) -> Result<(), Error> {
    
    let converted_url = url.replace("https://x.com/", "https://fxtwitter.com/");
    ctx.say(converted_url).await?;
    Ok(())
}

pub fn setup() -> Vec<poise::Command<crate::Data, Error>> {
    vec![xlinkconvert()]
}