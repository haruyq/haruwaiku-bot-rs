use chrono::{Datelike, Duration, Local, NaiveTime, TimeZone, Weekday};
use std::{
    collections::HashMap,
    env,
    error::Error,
    path::{Path, PathBuf},
};
use tokio::fs::{self};

use crate::reminder::model;
use model::{ReminderData, ReminderUpdate, RemindersMap};

pub struct ReminderUtil;

impl ReminderUtil {
    fn get_user_file_path(user_id: &u64) -> String {
        let save_dir =
            env::var("DATA_DIR").unwrap_or_else(|_| "/Common/Data".to_string()) + "/Reminders";
        format!("{}/{}.json", save_dir, user_id)
    }

    pub async fn read_dir() -> Result<Vec<PathBuf>, Box<dyn std::error::Error + Send + Sync>> {
        let path =
            env::var("DATA_DIR").unwrap_or_else(|_| "/Common/Data".to_string()) + "/Reminders";

        let mut paths = Vec::new();
        let mut entries = match fs::read_dir(&path).await {
            Ok(entries) => entries,
            Err(e) => {
                println!("Failed to read directory {:?}: {}", path, e);
                return Err(e.into());
            }
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            paths.push(path);
        }

        Ok(paths)
    }

    pub async fn read_user_reminders(
        user_id: &u64,
    ) -> Result<RemindersMap, Box<dyn std::error::Error + Send + Sync>> {
        let file_path = Self::get_user_file_path(user_id);
        let content = fs::read_to_string(&file_path).await?;
        let data: RemindersMap = serde_json::from_str(&content)?;
        Ok(data)
    }

    pub async fn read_json(
        path: &Path,
    ) -> Result<RemindersMap, Box<dyn std::error::Error + Send + Sync>> {
        let content = fs::read_to_string(path).await?;
        let data: RemindersMap = serde_json::from_str(&content)?;
        Ok(data)
    }

    async fn write_user_reminders(
        user_id: &u64,
        reminders: &RemindersMap,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let file_path = Self::get_user_file_path(user_id);
        let file = std::fs::File::create(file_path)?;
        serde_json::to_writer_pretty(file, reminders)?;
        Ok(())
    }

    pub async fn edit_reminders_json(
        user_id: &u64,
        reminder_name: &str,
        update: ReminderUpdate<'_>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut reminders = Self::read_user_reminders(user_id).await?;

        if let Some(reminder) = reminders.get_mut(reminder_name) {
            if let Some(v) = update.name {
                reminder.name = v.to_string();
            }
            if let Some(v) = update.channel_id {
                reminder.channel_id = v;
            }
            if let Some(v) = update.text {
                reminder.text = v.to_string();
            }
            if let Some(v) = update.is_per_day {
                reminder.is_per_day = v;
            }
            if let Some(v) = update.days {
                reminder.days = v;
            }
            if let Some(v) = update.time {
                reminder.time = v.to_string();
            }
            if let Some(v) = update.week {
                reminder.week = v.cloned();
            }
            if let Some(v) = update.next_reminder {
                reminder.next_reminder = v.to_string();
            }
            if let Some(v) = update.before_remind {
                reminder.before_remind = v.to_string();
            }
        }

        Self::write_user_reminders(user_id, &reminders).await?;
        Ok(())
    }

    pub async fn del_reminders_json(user_id: &u64) -> Result<(), Box<dyn Error + Send + Sync>> {
        let file_path = Self::get_user_file_path(user_id);
        std::fs::remove_file(file_path)?;
        Ok(())
    }

    pub async fn remove_reminder(
        user_id: &u64,
        name: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut reminders = Self::read_user_reminders(user_id).await?;
        reminders.remove(name);
        Self::write_user_reminders(user_id, &reminders).await?;
        Ok(())
    }

    pub async fn make_reminder(
        user_id: u64,
        name: &str,
        channel_id: u64,
        text: &str,
        is_per_day: bool,
        days: Option<u32>,
        time: &str,
        week: Option<&Weekday>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut reminders = Self::read_user_reminders(&user_id)
            .await
            .unwrap_or_else(|_| HashMap::new());

        let next_reminder = if is_per_day {
            let days = days.ok_or("days must be specified when is_per_day is true")?;

            let time_without_tz = time
                .split('+')
                .next()
                .or_else(|| time.split('-').next())
                .unwrap_or(time)
                .trim();

            let target_time = NaiveTime::parse_from_str(time_without_tz, "%H:%M:%S")
                .map_err(|e| format!("Failed to parse time '{}': {}", time, e))?;

            let now = Local::now();
            let today = now.date_naive();
            let today_target = today.and_time(target_time);

            let first_run = if now.time() >= target_time {
                today_target + Duration::days(days as i64)
            } else {
                today_target
            };

            Local
                .from_local_datetime(&first_run)
                .single()
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S%:z")
                .to_string()
        } else {
            let week = week.ok_or("week must be specified when is_per_day is false")?;

            let time_without_tz = time
                .split('+')
                .next()
                .or_else(|| time.split('-').next())
                .unwrap_or(time)
                .trim();

            let target_time = NaiveTime::parse_from_str(time_without_tz, "%H:%M:%S")
                .map_err(|e| format!("Failed to parse time '{}': {}", time, e))?;

            let now = Local::now();
            let current_weekday = now.weekday();
            let mut days_until_next =
                (week.num_days_from_monday() + 7 - current_weekday.num_days_from_monday()) % 7;
            if days_until_next == 0 && now.time() > target_time {
                days_until_next = 7;
            }
            let next_date =
                (now.date_naive() + Duration::days(days_until_next as i64)).and_time(target_time);
            Local
                .from_local_datetime(&next_date)
                .single()
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S%:z")
                .to_string()
        };

        let reminder_data = ReminderData {
            user_id,
            name: name.to_string(),
            channel_id,
            text: text.to_string(),
            is_per_day,
            days,
            time: time.to_string(),
            week: week.cloned(),
            next_reminder,
            before_remind: String::new(),
        };

        reminders.insert(name.to_string(), reminder_data);
        Self::write_user_reminders(&user_id, &reminders).await?;

        Ok(())
    }
}
