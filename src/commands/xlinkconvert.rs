use crate::{Context, Error};
use url::{Url};

#[poise::command(slash_command, guild_only)]
pub async fn xlinkconvert(
    ctx: Context<'_>,
    #[description = "変換するURL"]
    url: String,
) -> Result<(), Error> {
    let uri: Url = match Url::parse(&url) {
        Ok(u) => u,
        Err(_) => {
            ctx.say("無効なURLです。").await?;
            return Ok(());
        }
    };
    if uri.host().map_or(true, |h| h != url::Host::Domain("x.com")) {
        ctx.say("URLはX.comのものでなければなりません。").await?;
        return Ok(());
    }
    let converted_url = url.replace("https://x.com/", "https://fxtwitter.com/");
    ctx.say(converted_url).await?;
    Ok(())
}

pub fn setup() -> Vec<poise::Command<crate::Data, Error>> {
    vec![xlinkconvert()]
}