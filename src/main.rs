use anyhow::Result;
use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
struct Data {}
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod events;

#[tokio::main]
async fn main() -> Result<()> {
    let mut token = std::env::var("TOKEN").unwrap_or_default();
    if token.is_empty() {
        let mut input = String::new();
        println!("Please enter your bot token:");
        std::io::stdin().read_line(&mut input)?;
        token = input.trim().to_string();
    }

    let intents = serenity::GatewayIntents::default();

    let commands = {
        let mut v = Vec::new();
        v.extend(commands::ping::setup());
        v
    };

    let events = {
        events::on_ready::Handler
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(events)
        .await?;

    client.start().await?;
    Ok(())
}
