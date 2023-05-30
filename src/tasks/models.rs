use crate::utils::{Day, Priority};
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: i16,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub start: String,
    pub end: String,
    pub priority: Priority,
    pub one_off: bool,
    pub done: bool,
    pub days: Vec<Day>,
    pub crontab_line: Option<i16>,
}

impl Task {
    fn default() -> Self {
        Task {
            id: 0,
            title: String::new(),
            description: String::new(),
            due_date: String::new(),
            start: String::new(),
            end: String::new(),
            priority: Priority::Low,
            one_off: true,
            done: false,
            days: vec![],
            crontab_line: None,
        }
    }
    pub fn new() -> Self {
        Task::default()
    }
    pub fn id(mut self, id: i16) -> Self {
        self.id = id;
        self
    }

    pub fn title(mut self, title: Option<String>) -> Self {
        self.title = title.unwrap_or(self.title);
        self
    }

    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description.unwrap_or(self.description);
        self
    }

    pub fn due_date(mut self, due_date: Option<String>) -> Self {
        self.due_date = due_date.unwrap_or(self.due_date);
        self
    }

    pub fn start(mut self, start: Option<String>) -> Self {
        self.start = start.unwrap_or(self.start);
        self
    }

    pub fn end(mut self, end: Option<String>) -> Self {
        self.end = end.unwrap_or(self.end);
        self
    }

    pub fn priority(mut self, priority: Option<Priority>) -> Self {
        self.priority = priority.unwrap_or(self.priority);
        self
    }

    pub fn one_off(mut self, one_off: Option<bool>) -> Self {
        self.one_off = one_off.unwrap_or(self.one_off);
        self
    }

    pub fn done(mut self, done: Option<bool>) -> Self {
        self.done = done.unwrap_or(self.done);
        self
    }

    pub fn days(mut self, days: Option<Vec<Day>>) -> Self {
        self.days = days.unwrap_or(self.days);
        self
    }
    pub fn is_due_today(date_str: &str) -> bool {
        let now: NaiveDateTime = Local::now().naive_local();
        if let Ok(date) = NaiveTime::parse_from_str(date_str, "%H:%M") {
            return date.hour() == now.hour() && date.minute() == now.minute();
        }
        if let Ok(datetime) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M") {
            return datetime == now;
        }
        if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            return date == now.date();
        }
        false
    }
    pub fn duration(&self) -> f64 {
        let start_date = match NaiveTime::parse_from_str(&self.start, "%H:%M") {
            Ok(value) => value,
            Err(err) => panic!("oupsi, {:?}", err),
        };
        let end_date = match NaiveTime::parse_from_str(&self.end, "%H:%M") {
            Ok(value) => value,
            Err(err) => panic!("oupsi, {:?}", err),
        };
        let duration = end_date.signed_duration_since(start_date);
        let duration_hours = duration.num_hours() as f64 + (duration.num_minutes() as f64 / 60.0);

        duration_hours
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        let task = Task::new()
            .start(Some("10:30".to_string()))
            .end(Some("14:45".to_string()));
        assert_eq!(task.duration(), 4.25);
    }
}
