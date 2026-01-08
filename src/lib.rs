use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

impl Todo {
    pub fn new(id: u16, todo: String) -> Todo {
        Todo {
            id: id.to_string(),
            name: todo,
            created_at: Utc::now().to_string(),
        }
    }
}

pub fn add_new_todo(todo: &str, current_todo_list: &mut Vec<Todo>) -> std::io::Result<()> {
    let new_id: u16 = (current_todo_list.len() + 1) as u16;
    let todo = Todo::new(new_id, String::from(todo));

    current_todo_list.push(todo);

    write_to_file(current_todo_list)
}

pub fn list_all_todos(current_todo_list: &mut Vec<Todo>) {
    if current_todo_list.len() > 0 {
        for todo in current_todo_list {
            println!("{}. {}", todo.id, todo.name);
        }

        return;
    }

    println!("There is no todo")
}

pub fn remove_task_by_id(id: &str, current_todo_list: &mut Vec<Todo>) -> std::io::Result<()> {
    if current_todo_list.len() <= 0 {
        println!("Invalid todo id.")
    }

    let mut new_todo_list: Vec<Todo> = current_todo_list
        .iter()
        .filter(|todo| todo.id != id)
        .cloned()
        .collect();

    write_to_file(&mut new_todo_list)
}

pub fn get_current_todo_list() -> Vec<Todo> {
    let path = todo_file_path();
    match fs::read_to_string(path) {
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

pub fn reset_todo_ids(current_todo_list: &mut Vec<Todo>) -> &mut Vec<Todo> {
    for (i, todo) in current_todo_list.iter_mut().enumerate() {
        let new_index = (i + 1) as u16;
        todo.id = new_index.to_string();
    }

    current_todo_list
}

fn write_to_file(todo_list: &mut Vec<Todo>) -> std::io::Result<()> {
    let path = todo_file_path();
    let file = File::create(path)?;

    let todo_json = json!(todo_list);

    serde_json::to_writer(file, &todo_json)?;

    Ok(())
}

fn todo_file_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME directory not set");

    let mut path = PathBuf::from(home);
    path.push(".local");
    path.push("share");
    path.push("cli");
    path.push("todo");

    // Ensure directories exist
    fs::create_dir_all(&path).expect("Failed to create todo data directory");

    path.push("todo-list.json");
    path
}
