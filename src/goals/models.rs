use std::collections::HashMap;

use crate::tasks::Task;
use crate::utils::Priority;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Goal {
    pub id: i16,
    pub title: String,
    pub why: String,
    pub how: String,
    pub notes: String,
    pub priority: Priority,
    pub tasks: HashMap<i16, Task>,
    pub hours_per_day: Option<f64>,
}

impl Goal {
    fn default() -> Self {
        Goal {
            id: 0,
            title: String::new(),
            why: String::new(),
            how: String::new(),
            notes: String::new(),
            priority: Priority::Low,
            tasks: HashMap::new(),
            hours_per_day: None,
        }
    }
    pub fn new() -> Self {
        Goal::default()
    }
    pub fn add_task(mut self, task: Task) -> Self {
        self.tasks.entry(task.id).or_insert(task);
        self
    }
    fn calculate_hours_per_day(mut self) -> Self {
        self.tasks.iter().for_each(|(_, task)| {
            self.hours_per_day = Some(self.hours_per_day.unwrap_or(0.0) + task.duration())
        });
        self
    }
}
