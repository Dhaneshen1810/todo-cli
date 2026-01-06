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
            id: String::from("1"),
            name: todo,
            created_at: Utc::now().to_string(),
        }
    }
}

fn read_from_file() {
    println!("Todo list:");
    let contents =
        fs::read_to_string("todo-list.json").expect("Should have been able to read the file");
    let content_str = contents.as_str();
    let data: Todo = serde_json::from_str(content_str).expect("Invalid JSON");
    println!("- {}", data.name);
}

fn write_to_file(todo: String) -> std::io::Result<()> {
    let todo = Todo::new(todo);
    let file = File::create("todo-list.json")?;

    let todo_json = json!({
        "id": todo.id,
        "name": todo.name,
        "created_at": todo.created_at.to_string()
    });

    serde_json::to_writer(file, &todo_json)?;

    Ok(())
}
