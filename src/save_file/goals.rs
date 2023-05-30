use super::FileSaver;
use crate::goals::Goal;
use crate::utils::{Day, Priority};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalsFile {
    pub goals: HashMap<i16, Goal>,
}

impl GoalsFile {
    fn add(
        &mut self,
        title: String,
        description: Option<String>,
        due_date: Option<String>,
        start: Option<String>,
        end: Option<String>,
        priority: Option<Priority>,
        one_off: Option<bool>,
        done: Option<bool>,
        days: Option<Vec<Day>>,
    ) -> i16 {
        let task: Task = Task::new()
            .id(self.get_latest_id())
            .title(Some(title))
            .description(description)
            .due_date(due_date)
            .start(start)
            .end(end)
            .priority(priority)
            .one_off(one_off)
            .done(done)
            .days(days);
        if task.due_date.is_empty() && task.one_off {
            panic!("A task that is one off has to have a due_date")
        }
        self.tasks.entry(task.id).or_insert(task);
        self.save_changes();
        1
    }
    fn update(
        mut self,
        id: i16,
        title: Option<String>,
        description: Option<String>,
        due_date: Option<String>,
        start: Option<String>,
        end: Option<String>,
        priority: Option<Priority>,
        one_off: Option<bool>,
        done: Option<bool>,
        days: Option<Vec<Day>>,
    ) -> i16 {
        self.tasks.entry(id).and_modify(|todo| {
            *todo = todo
                .clone()
                .title(title)
                .description(description)
                .due_date(due_date)
                .start(start)
                .end(end)
                .priority(priority)
                .one_off(one_off)
                .done(done)
                .days(days);
        });

        self.save_changes();
        1
    }
}

impl FileSaver for GoalsFile {
    fn read(&mut self, _id: Option<i16>, _title: Option<String>) -> i16 {
        println!("{}", serde_json::to_string_pretty(&self.tasks).unwrap());
        1
    }

    fn delete(&mut self, id: Option<i16>, title: Option<String>) -> i16 {
        if id.is_none() && title.is_none() {
            eprintln!("Error: Either `id` or `title` must be provided to delete a Todo.");
            return 1;
        }

        // Check if `id` is provided and delete the corresponding `Todo`
        if let Some(id) = id {
            if let Some(todo) = self.tasks.remove(&id) {
                self.save_changes();
                println!("Deleted Todo: {:?}", todo);
                return id;
            }
        }

        // Check if `title` is provided and delete the corresponding `Todo`
        if let Some(title) = title {
            if let Some((&id, _)) = self.tasks.iter().find(|(_, t)| t.title == title) {
                if let Some((rem_id, rem_todo)) = self.tasks.remove_entry(&id) {
                    self.save_changes();
                    println!("Deleted Todo: {:?}", rem_todo);
                    return rem_id;
                }
            }
        }

        eprintln!("Error: Todo not found.");
        return 1;
    }
    fn file_name() -> String {}
}
