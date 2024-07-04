extern crate ctrlc;
extern crate serde;

use std::{fs::File, io::{self, Read, Write}, time::{SystemTime, UNIX_EPOCH}};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};


#[derive(Serialize, Deserialize)]
struct TodoEntry {
    timestamp_seconds : u64,
    entry_text        : String,
    priority          : i32,
}

impl TodoEntry {
    pub fn new(timestamp_seconds : u64, entry_text : String, priority : i32) -> Self {
        Self {
            timestamp_seconds,
            entry_text,
            priority,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    entries : Vec<TodoEntry>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            entries : Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry : TodoEntry) {
        self.entries.push(entry);
    }

    pub fn sort(&mut self) {
        //Sort in ascending priority order
        self.entries.sort_by(|a, b| a.priority.cmp(&b.priority))
    }
}

fn main() -> io::Result<()> {

    let running = Arc::new(AtomicBool::new(true));

    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }) .expect("Error setting ctrl-c handler");


    let mut buf = String::new();
    let mut list = TodoList::new();

    let file = File::open("list.bin");

    match file {
        Ok(mut f) => {

            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).expect("Contents of serialized file invalid");
            list = deserialize(&mut buffer).expect("Failed to create list from buffer");
        },
        Err(_) => {
            println!("Serialized List not found");
        }
    };


    while running.load(Ordering::SeqCst) {

        buf.clear();

        prompt("Enter Command", &mut buf);

        match buf.to_lowercase().trim() {
            "l" | "list" => print_list(&mut list),
            "a" | "add"  => add_dialogue(&mut list),
            "e" | "exit" => {
                save_list(&list);
                return Ok(())
            },
            _ => print_help(),
        }
    }

    println!("Exiting Gracefully");
    save_list(&list);

    Ok(())
}

//TODO:
// 1. Write own atomic bools/ints
// 2. Serialize List

fn prompt(prompt : &str, input : &mut String) {
    print!("{}: ", prompt);
    io::stdout().flush().expect("Failed To Flush stdout");

    io::stdin().read_line(input).expect("Failed To Read stdin");
}

fn save_list(list : &TodoList) {

    let bin = serialize(list).expect("Binary Serialization Failed");
    let mut file = File::create("list.bin").expect("Failed to create binary file");
    file.write_all(&bin).expect("Failed to write binary file");
}

fn print_help() {

    println!("--- TODO CLI ---");
    println!("l | list => print todo list");
    println!("a | add  => add new list entry");
    println!("h | help => print help menu");
    println!("e | exit => exit tool")

}
fn print_list(list : &mut TodoList) {
    if list.entries.len() == 0 {
        println!("--- No Tasks Yet! ---");
        return;
    }

    list.sort();
    println!("--- TODO ---");
    for (pri, entry) in list.entries.iter().enumerate() {
        print!("{}: {}", pri + 1, entry.entry_text);
    }
}

fn add_dialogue(list : &mut TodoList) {
    println!("Adding Item!");

    let mut input = String::new();

    prompt("Enter Priority", &mut input);

    let pri = match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {

            println!("INVALID PRIORITY: You entered {}", input);
            return;
        }
    };
    if pri < 1 {
        println!("INVALID PRIORITY: Please Enter a number greater than 0.");
        return;
    }

    input.clear();

    prompt("Enter Details", &mut input);
    println!("Details = {}", input);

    let curr_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get time since epoch")
        .as_secs();

    list.add_entry(TodoEntry::new(curr_time, input, pri));
}