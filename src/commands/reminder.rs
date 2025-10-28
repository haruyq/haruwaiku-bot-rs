use crate::{Context, Error};
use chrono::Weekday;
use discord_bot_rs::reminder::utils::ReminderUtil;
use poise::{CreateReply, serenity_prelude as serenity};
use serenity::model::channel::Channel;

#[derive(poise::ChoiceParameter)]
pub enum WeekdayParam {
    #[name = "Monday"]
    Monday,
    #[name = "Tuesday"]
    Tuesday,
    #[name = "Wednesday"]
    Wednesday,
    #[name = "Thursday"]
    Thursday,
    #[name = "Friday"]
    Friday,
    #[name = "Saturday"]
    Saturday,
    #[name = "Sunday"]
    Sunday,
}

/// Set a reminder for a specific time and date.
#[poise::command(slash_command)]
pub async fn reminder_set(
    ctx: Context<'_>,
    name: String,
    text: String,
    channel: Channel,
    #[description = "%H:%M:%S+<TZ> format (example: 12:00:00+09:00)"] time: String,
    days: Option<u32>,
    weekday: Option<WeekdayParam>,
) -> Result<(), Error> {
    if days.is_none() && weekday.is_none() && days.is_some() && weekday.is_some() {
        ctx.say("Argument error").await?;
        return Ok(());
    }

    ctx.defer().await?;

    let is_per_day = days.is_some();
    let days_val = days.unwrap_or(0);

    let day_weekday = weekday.map(|w| match w {
        WeekdayParam::Sunday => Weekday::Sun,
        WeekdayParam::Monday => Weekday::Mon,
        WeekdayParam::Tuesday => Weekday::Tue,
        WeekdayParam::Wednesday => Weekday::Wed,
        WeekdayParam::Thursday => Weekday::Thu,
        WeekdayParam::Friday => Weekday::Fri,
        WeekdayParam::Saturday => Weekday::Sat,
    });

    let user_id = ctx.author().id.get();
    let channel_id = channel.id().get();

    match ReminderUtil::make_reminder(
        user_id,
        &name,
        channel_id,
        &text,
        is_per_day,
        Some(days_val),
        &time,
        day_weekday.as_ref(),
    )
    .await
    {
        Ok(_) => {
            ctx.send(CreateReply::default().content("Reminder created successfully."))
                .await?;
        }
        Err(err) => {
            ctx.send(CreateReply::default().content(format!("Failed to create reminder: {}", err)))
                .await?;
        }
    }

    Ok(())
}

/// Delete a reminder
#[poise::command(slash_command)]
async fn reminder_delete(
    ctx: Context<'_>,
    #[description = "Name of the reminder"] name: String,
) -> Result<(), Error> {
    let user_id = ctx.author().id.get();

    match ReminderUtil::remove_reminder(&user_id, &name).await {
        Ok(_) => {
            ctx.send(CreateReply::default().content("Reminder deleted successfully."))
                .await?;
        }
        Err(err) => {
            ctx.send(CreateReply::default().content(format!("Failed to delete reminder: {}", err)))
                .await?;
        }
    }

    Ok(())
}

pub fn setup() -> Vec<poise::Command<crate::Data, Error>> {
    vec![reminder_set(), reminder_delete()]
}
