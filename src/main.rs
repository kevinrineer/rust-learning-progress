/*
* 1. Show TODO item(s) via show
* 2. Get new TODO item(s) via input
* 3. Check off TODO item(s) via check
* 4. Remove TODO item(s) via remove
* 5. Always save to file
*/

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(visible_alias = "new", alias = "n")]
    Add {
        #[arg(help = "Task description")]
        task: String,
    },
    #[clap(visible_alias = "ls", alias = "show")]
    List,
    Done {
        #[arg(help = "Task ID to mark done")]
        id: usize,
    },
    #[clap(alias = "del", alias = "d")]
    Delete {
        #[arg(help = "Task ID to delete")]
        id: usize,
    },
}

fn delete_todo(id: usize) {
    println!("{} Gone", id);
}
fn add_todo(description: String) {
    println!("Added {}", description);
}
fn mark_todo_done(id: usize) {
    println!("{} Done", id);
}
fn list_todos() {
    println!("TODOs");
}

fn main() {
    let subcommands = Cli::parse();

    match subcommands.command {
        Commands::List => list_todos(),
        Commands::Delete { id } => delete_todo(id),
        Commands::Add { task } => add_todo(task),
        Commands::Done { id } => mark_todo_done(id),
    }
}
