use chrono::Utc;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    todo: Option<String>,

    // list all todos
    #[arg(short)]
    l: bool,
    // #[arg(short, long, default_value_t = 1)]
    // list: u8,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.l {
        read_from_file();
        return Ok(());
    }

    if let Some(todo) = args.todo {
        match write_to_file(todo) {
            Ok(()) => println!("Success!"),
            Err(e) => println!("Failed to write to file: {}", e),
        };
    } else {
        eprintln!("Invalid command.");
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: String,
    name: String,
    created_at: String,
}

impl Todo {
    fn new(todo: String) -> Todo {
        Todo {
            id: String::from("2"),
            name: todo,
            created_at: Utc::now().to_string(),
        }
    }
}

fn read_from_file() {
    match fs::read_to_string("todo-list.json") {
        Ok(contents) => {
            println!("Todo list:");

            let content_str = contents.as_str();
            let data: Vec<Todo> = serde_json::from_str(content_str).expect("Invalid JSON");

            for todo in data {
                println!("{}. {}", todo.id, todo.name);
            }
        }
        Err(_e) => println!("There is currently no todo."),
    }
}

fn write_to_file(todo: String) -> std::io::Result<()> {
    let todo = Todo::new(todo);
    let mut new_todo_list: Vec<Todo> = get_current_todo_list();
    new_todo_list.push(todo);
    let file = File::create("todo-list.json")?;

    let todo_json = json!(new_todo_list);

    serde_json::to_writer(file, &todo_json)?;

    Ok(())
}

fn get_current_todo_list() -> Vec<Todo> {
    match fs::read_to_string("todo-list.json") {
        Ok(content_str) => match serde_json::from_str(&content_str) {
            Ok(content) => {
                let current_todo_list: Vec<Todo> = content;
                current_todo_list
            }
            Err(_e) => Vec::new(),
        },
        Err(_e) => Vec::new(),
    }
}
