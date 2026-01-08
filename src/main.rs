use clap::{Args, Parser, Subcommand};
use todo::{add_new_todo, list_all_todos, remove_task_by_id};

#[derive(Parser)]
#[command(author = "Dhaneshen Moonian", version="0.0.1", about, long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Add new todo
    Add(Add),
    // List all todos
    List,
    // Remove todo
    Rm(Remove),
}

#[derive(Args)]
struct Add {
    string: Option<String>,
}

#[derive(Args)]
struct Remove {
    id: Option<String>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add(name)) => match &name.string {
            Some(name) => match add_new_todo(name) {
                Ok(_) => println!("New todo added."),
                Err(_e) => println!("Failed to add todo"),
            },
            None => println!("Please provide a todo"),
        },
        Some(Commands::List) => list_all_todos(),
        Some(Commands::Rm(todo)) => match &todo.id {
            Some(id) => match remove_task_by_id(id) {
                Ok(_) => {}
                Err(_e) => println!("Failed to remove todo"),
            },
            None => println!("Failed to remove todo."),
        },
        None => {}
    }

    Ok(())
}
