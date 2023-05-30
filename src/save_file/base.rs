use crate::utils::{Day, Priority};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;

pub trait FileSaver {
    pub fn read(&mut self, _id: Option<i16>, _title: Option<String>) -> i16 {}

    pub fn delete(&mut self, id: Option<i16>, title: Option<String>) -> i16 {}

    pub fn get_or_create() -> Self {}

    fn save_changes(&mut self) {
        let mut file = TasksFile::open_or_create_file();
        file.set_len(0).unwrap();
        file.write_all(serde_json::to_string_pretty(self).unwrap().as_bytes())
            .expect("something");
    }

    fn open_or_create_file() -> File {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(TasksFile::get_file_path())
        {
            Ok(value) => value,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        }
    }

    fn get_file_path(self) -> String {
        let folder_path = format!("{}/.todors", self::home_dir());
        if !Path::new(&folder_path).exists() {
            if let Err(err) = create_dir(&folder_path) {
                panic!("Failed to create folder: {}", err);
            };
        };
        format!("{}/{}.json", folder_path, self::file_name())
    }

    fn file_name() -> String {}

    fn home_dir() -> String {
        match env::var("HOME") {
            Ok(value) => value,
            Err(err) => panic!("Failed to retrieve home directory, {:?}", err),
        }
    }

    fn get_latest_id(&mut self) -> i16 {}
}
