use chrono::Weekday;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReminderData {
    pub user_id: u64,
    pub name: String,
    pub channel_id: u64,
    pub text: String,
    pub is_per_day: bool,
    pub days: Option<u32>,
    pub time: String,
    pub week: Option<Weekday>,
    pub next_reminder: String,
    pub before_remind: String,
}

#[derive(Default)]
pub struct ReminderUpdate<'a> {
    pub name: Option<&'a str>,
    pub channel_id: Option<u64>,
    pub text: Option<&'a str>,
    pub is_per_day: Option<bool>,
    pub days: Option<Option<u32>>,
    pub time: Option<&'a str>,
    pub week: Option<Option<&'a Weekday>>,
    pub next_reminder: Option<&'a str>,
    pub before_remind: Option<&'a str>,
}

pub type RemindersMap = HashMap<String, ReminderData>;
