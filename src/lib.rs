use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::fs::File;

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    id: String,
    name: String,
    created_at: String,
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

pub fn add_new_todo(todo: &str) -> std::io::Result<()> {
    let mut new_todo_list: Vec<Todo> = get_current_todo_list();

    let new_id: u16 = (new_todo_list.len() + 1) as u16;
    let todo = Todo::new(new_id, String::from(todo));

    new_todo_list.push(todo);
    let file = File::create("todo-list.json")?;

    let todo_json = json!(new_todo_list);

    serde_json::to_writer(file, &todo_json)?;

    Ok(())
}

pub fn list_all_todos() {
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

pub fn remove_task_by_id(id: &str) -> std::io::Result<()> {
    let current_todo_list = get_current_todo_list();

    if current_todo_list.len() <= 0 {
        println!("Invalid todo id.")
    }

    let mut new_todo_list: Vec<Todo> = current_todo_list
        .iter()
        .filter(|todo| todo.id != id)
        .cloned()
        .collect();

    let formatted_todo_list = reset_todo_ids(&mut new_todo_list);

    // Replace todo file
    let file = File::create("todo-list.json")?;

    let todo_json = json!(formatted_todo_list);

    serde_json::to_writer(file, &todo_json)?;
    println!("Todo removed.");

    Ok(())
}

pub fn get_current_todo_list() -> Vec<Todo> {
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

pub fn reset_todo_ids(current_todo_list: &mut Vec<Todo>) -> &mut Vec<Todo> {
    for (i, todo) in current_todo_list.iter_mut().enumerate() {
        let new_index = (i + 1) as u16;
        todo.id = new_index.to_string();
    }

    current_todo_list
}
