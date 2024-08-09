extern crate ctrlc;
extern crate serde;

use std::fs::File;
use std::io::{Read, Write};

use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoEntry {
    // timestamp_seconds : u64,
    entry_text: String,
    priority: i32,
    completed: bool,
}

impl TodoEntry {
    pub fn new(completed: bool, entry_text: String, priority: i32) -> Self {
        Self {
            // timestamp_seconds,
            completed,
            entry_text,
            priority,
        }
    }
    pub fn completed(&self) -> bool {
        self.completed
    }
    pub fn text(&self) -> &String {
        &self.entry_text
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    entries: Vec<TodoEntry>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn from_file() -> Self {
        let fd = File::open("list.bin");
        match fd {
            Ok(mut f) => {
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).expect("Failed to read list file to end");
                deserialize(&buffer).expect("Failed to create list from buffer")
            }
            Err(_) => {
                println!("Serialized List Not Found!");
                Self::new()
            }
        }
    }

    pub fn save(&self) {
        let bin = serialize(self).expect("Failed to save todo list");
        let mut file = File::create("list.bin").expect("Failed to create list file");
        file.write_all(&bin).expect("Failed to write list binary");
    }

    pub fn entries(&self) -> &Vec<TodoEntry> {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: TodoEntry) {
        self.entries.push(entry);
    }

    #[allow(unused)]
    pub fn sort(&mut self) {
        //Sort in ascending priority order
        self.entries.sort_by(|a, b| a.priority.cmp(&b.priority))
    }
}