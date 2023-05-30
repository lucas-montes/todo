use super::{Task, TasksFile};
use crate::utils::Priority;
use chrono::prelude::*;
use chrono::Local;
use notify_rust::{Notification, Timeout};

pub fn handle_function(function: &str) -> i16 {
    match function {
        "read_one_off" => read_one_off(),
        "save_repetitives_one" => save_repetitives_one(),
        _ => check_tasks(),
    };
    0
}
fn check_tasks() {
    for (_id, task) in TasksFile::get_or_create().tasks.iter() {
        if !task.done && (check_one_off(task) | check_repetitif(task)) {
            let due_date: &str = if task.start.is_empty() {
                &task.due_date
            } else {
                &task.start
            };
            notify(
                &create_notification_summary(&task.title, due_date),
                &task.description,
                &match_priority(&task.priority),
            )
        }
    }
}
fn check_one_off(task: &Task) -> bool {
    task.one_off && Task::is_due_today(&task.due_date)
}
fn check_repetitif(task: &Task) -> bool {
    task.days
        .iter()
        .any(|day| day.to_digit() as u32 == Local::now().weekday().number_from_monday())
        && Task::is_due_today(&task.start)
}
fn read_one_off() {
    // this will be called from a cron or a daemon.
    // it will check all the one_off tasks
}
fn save_repetitives_one() {
    // all the tasks that are repetitives will be created into a cronjob
    // once the tasks has been done, if the tasks hasn0t to be done againt, delete it?
}
fn create_notification_summary(title: &str, due_date: &str) -> String {
    format!("The task {title} is due {due_date}")
}
fn match_priority(priority: &Priority) -> String {
    match priority {
        Priority::High => String::from("dialog-error"),
        Priority::Medium => String::from("dialog-warning"),
        Priority::Low => String::from("dialog-information"),
    }
}
fn notify(summary: &str, body: &str, icon: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .icon(icon)
        .timeout(Timeout::Never)
        .show()
        .unwrap();
}
