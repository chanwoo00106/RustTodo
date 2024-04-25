use clap::Parser;
use todo_cli::commands::{Command, TodoCliOptions};

fn main() {
    let command = TodoCliOptions::parse();

    match command.command {
        Command::Add(todo) => {
            println!("{}", todo.todo);
        }
        Command::Remove(todo) => {
            println!("{}", todo.todo);
        }
    }
}
