use crate::todo::Day;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cron {
    pub minute: String,
    pub hour: String,
    pub day_of_month: String,
    pub day_of_week: String,
    pub month: String,
}

impl Cron {
    fn default() -> Self {
        Cron {
            minute: String::from("*"),
            hour: String::from("*"),
            day_of_month: String::from("*"),
            day_of_week: String::from("*"),
            month: String::from("*"),
        }
    }
    pub fn new() -> Self {
        Cron::default()
    }
    pub fn save(self) {}
    pub fn to_string(&self) -> String {
        // Min Hour Day-of-month Month Day-of-week
        // 0-59 0-23 1-31/30 1-12 0-6(0= Sunday, 6=Saturday)
        format!(
            "{{&self.minute}} {{&self.hour}} {{&self.day_of_month}} {{&self.month}} {{&self.day_of_week}}"
        )
    }
    pub fn minute(mut self, minute: Option<String>) -> Self {
        self.minute = minute.unwrap_or(self.minute);
        self
    }
    pub fn hour(mut self, hour: Option<String>) -> Self {
        self.hour = hour.unwrap_or(self.hour);
        self
    }
    pub fn month(mut self, month: Option<String>) -> Self {
        self.month = month.unwrap_or(self.month);
        self
    }
    pub fn day_of_month(mut self, day_of_month: Option<String>) -> Self {
        self.day_of_month = day_of_month.unwrap_or(self.day_of_month);
        self
    }
    pub fn day_of_week(mut self, days: Option<Vec<Day>>) -> Self {
        self.day_of_week = match days {
            Some(value) => Cron::days_to_cron(&value),
            None => self.day_of_week,
        };
        self
    }
    fn custom_cron(
        days: Vec<Day>,
        hour: String,
        minute: String,
        day_of_month: String,
        month: String,
    ) -> Cron {
        let mut cron = Cron::default();
        cron.hour = hour;
        cron.minute = minute;
        cron.month = month;
        cron.day_of_week = Cron::days_to_cron(&days);
        cron.day_of_month = day_of_month;
        cron
    }
    fn daily_cron(hour: String, minute: String) -> Cron {
        let mut cron = Cron::default();
        cron.hour = hour;
        cron.minute = minute;
        cron
    }
    fn weekly_cron(days: Vec<Day>, hour: String, minute: String) -> Cron {
        let mut cron = Cron::default();
        cron.hour = hour;
        cron.minute = minute;
        cron.day_of_week = Cron::days_to_cron(&days);
        cron
    }
    fn monthly_cron(day_of_month: String, hour: String, minute: String) -> Cron {
        let mut cron = Cron::default();
        cron.hour = hour;
        cron.minute = minute;
        cron.day_of_month = day_of_month;
        cron
    }
    fn days_to_cron(days: &[Day]) -> String {
        days.iter()
            .map(|day| match day {
                Day::Monday => "1",
                Day::Tuesday => "2",
                Day::Wednesday => "3",
                Day::Thursday => "4",
                Day::Friday => "5",
                Day::Saturday => "6",
                Day::Sunday => "0",
            })
            .collect::<Vec<&str>>()
            .join(",")
    }
}

fn save_cronjon() {
    // Once saved, get the line number and save it with the task
    let cron_command = "env XDG_RUNTIME_DIR=/run/user/$(id -u) /usr/bin/notify-send 'Hello, Rust!'";
    let cron_schedule = "* * * * *"; // Every minute

    // Get the current user's crontab
    let output = Command::new("crontab").arg("-l").output().unwrap();

    if output.status.success() {
        let existing_crontab = String::from_utf8_lossy(&output.stdout);
        let mut new_crontab = existing_crontab.to_string();

        // Append the new cron job entry
        new_crontab.push_str(&format!("\n{} {}\n", cron_schedule, cron_command));

        // Set the new crontab
        let mut child = Command::new("crontab")
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(new_crontab.as_bytes())
            .unwrap();
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Failed to read crontab: {}", error_message);
    }
}
