use anyhow::Result;
use dotenvy::dotenv;
use poise::serenity_prelude as serenity;
use std::{env, path::Path};

type Error = Box<dyn std::error::Error + Send + Sync>;
struct Data {}
type Context<'a> = poise::Context<'a, Data, Error>;

mod commands;
mod events;

#[tokio::main]
async fn main() -> Result<()> {
    if cfg!(target_os = "windows") {
        println!("Working on Windows");
        dotenv().expect("Failed to load configuration.");
    } else {
        println!("Working on Linux");

        let reminder_dir =
            env::var("DATA_DIR").unwrap_or_else(|_| "/Common/Data".to_string()) + "/Reminders";
        if !Path::new(&reminder_dir).exists() {
            tokio::fs::DirBuilder::new()
                .recursive(true)
                .create(reminder_dir)
                .await?;
        }
    }

    let token = env::var("TOKEN").unwrap_or_default();
    let intents = serenity::GatewayIntents::default();

    let commands = {
        let mut v = Vec::new();
        v.extend(commands::ping::setup());
        v.extend(commands::translate::setup());
        v.extend(commands::xlinkconvert::setup());
        v.extend(commands::nuke::setup());
        v.extend(commands::slowmode::setup());
        v
    };

    let events = { events::on_ready::Handler };

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
