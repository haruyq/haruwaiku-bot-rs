use crate::{Context, Error};
use serenity::all::{ChannelId, Colour, CreateChannel, GuildId, Mentionable};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};
use serenity::http::Http;
use serenity::model::channel::{Channel, GuildChannel};

pub async fn duplicate_channel(
    channel_id: ChannelId,
    guild_id: GuildId,
    http: &Http,
) -> Result<GuildChannel, serenity::Error> {
    let channel = channel_id.to_channel(http).await?;
    let guild_channel = match channel {
        Channel::Guild(c) => c,
        _ => {
            return Err(serenity::Error::Other("Not a guild channel"));
        }
    };

    let mut builder = CreateChannel::new(guild_channel.name)
        .kind(guild_channel.kind)
        .topic(guild_channel.topic.clone().unwrap_or_default())
        .nsfw(guild_channel.nsfw)
        .position(guild_channel.position);

    if let Some(parent_id) = guild_channel.parent_id {
        builder = builder.category(parent_id);
    }

    builder = builder.permissions(guild_channel.permission_overwrites.clone());

    let new_channel = guild_id.create_channel(http, builder).await?;

    Ok(new_channel)
}

/// Resets all logs for the specified channel.
#[poise::command(slash_command, required_permissions = "MANAGE_CHANNELS")]
pub async fn nuke(ctx: Context<'_>) -> Result<(), Error> {
    let http = ctx.http();
    let channel_id = ctx.channel_id();
    let guild_id = ctx.guild_id().ok_or("Command must be used in a guild")?;

    match duplicate_channel(channel_id, guild_id, http).await {
        Ok(new_channel) => {
            let old_channel = channel_id.to_channel(http).await?;
            old_channel.delete(http).await?;

            let embed = CreateEmbed::new()
                .description(format!("♻️ Channel {} has cleared.", new_channel.mention(),))
                .color(Colour::from_rgb(55, 255, 119))
                .author(
                    CreateEmbedAuthor::new(ctx.author().display_name())
                        .icon_url(ctx.author().avatar_url().unwrap_or_default()),
                );

            new_channel
                .send_message(http, serenity::builder::CreateMessage::new().embed(embed))
                .await?;
        }
        Err(err) => {
            println!("Error in nuking channel: {}", err);
        }
    };

    Ok(())
}

pub fn setup() -> Vec<poise::Command<crate::Data, Error>> {
    vec![nuke()]
}
