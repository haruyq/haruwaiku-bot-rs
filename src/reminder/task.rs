use crate::reminder::{model, utils};
use chrono::Local;
use model::ReminderUpdate;
use serenity::all::{
    ChannelId, CreateAllowedMentions, CreateEmbed, CreateEmbedAuthor, CreateMessage, UserId,
};
use serenity::http::Http;
use utils::ReminderUtil;

pub struct Task;

impl Task {
    pub async fn run(http: &Http) {
        let paths = match ReminderUtil::read_dir().await {
            Ok(paths) => paths,
            Err(e) => {
                println!("Failed to read directory: {}", e);
                return;
            }
        };

        for path in paths {
            match ReminderUtil::read_json(&path).await {
                Ok(reminders) => {
                    for (reminder_name, data) in reminders {
                        let now = chrono::Local::now();

                        let next_run_time = match chrono::DateTime::parse_from_str(
                            &data.next_reminder,
                            "%Y-%m-%d %H:%M:%S%:z",
                        ) {
                            Ok(dt) => dt.with_timezone(&Local),
                            Err(e) => {
                                println!(
                                    "Failed to parse next_reminder for {}: {}",
                                    reminder_name, e
                                );
                                continue;
                            }
                        };

                        if now < next_run_time {
                            continue;
                        }

                        let channel_id = ChannelId::new(data.channel_id);
                        let user_id = UserId::new(data.user_id);
                        let embed = CreateEmbed::new().author(CreateEmbedAuthor::new(format!(
                            "Reminder Name: {}",
                            data.name,
                        )));
                        let content = CreateMessage::new()
                            .content(data.text)
                            .allowed_mentions(CreateAllowedMentions::new().users([user_id]))
                            .embed(embed);

                        match channel_id.send_message(http, content).await {
                            Ok(_) => {
                                println!(
                                    "Reminder sent: {} for user {}",
                                    reminder_name, data.user_id
                                );

                                let next_reminder = if data.is_per_day {
                                    (next_run_time
                                        + chrono::Duration::days(data.days.unwrap() as i64))
                                    .format("%Y-%m-%d %H:%M:%S%:z")
                                    .to_string()
                                } else {
                                    (next_run_time + chrono::Duration::days(7))
                                        .format("%Y-%m-%d %H:%M:%S%:z")
                                        .to_string()
                                };

                                let before_remind = now.format("%Y-%m-%d %H:%M:%S%:z").to_string();

                                match ReminderUtil::edit_reminders_json(
                                    &data.user_id,
                                    &reminder_name,
                                    ReminderUpdate {
                                        before_remind: Some(&before_remind),
                                        next_reminder: Some(&next_reminder),
                                        ..Default::default()
                                    },
                                )
                                .await
                                {
                                    Ok(_) => {
                                        println!(
                                            "Updated next_reminder to {} for reminder {}",
                                            next_reminder, reminder_name
                                        );
                                    }
                                    Err(e) => {
                                        println!("Failed to edit reminders JSON: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Failed to send message to channel {}: {}", channel_id, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to read JSON file {:?}: {}", path, e);
                }
            }
        }
    }
}
