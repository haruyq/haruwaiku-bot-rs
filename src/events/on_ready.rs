use poise::serenity_prelude as serenity;
use ::serenity::async_trait;
use crate::Error;

pub struct Handler;

#[async_trait]
impl serenity::EventHandler for Handler {
    async fn ready(&self, ctx: serenity::Context, ready: serenity::Ready) {
        if let Err(e) = on_ready(&ctx, &ready).await {
            eprintln!("Error in on_ready: {:?}", e);
        }
    }
}

pub async fn on_ready(
    _ctx: &serenity::Context,
    ready: &serenity::Ready,
) -> Result<(), Error> {
    let discriminator = ready.user.discriminator.map(|d| d.get().to_string()).unwrap_or("None".to_string());
    println!("ログインしました: {}#{}", ready.user.name, discriminator);
    Ok(())
}