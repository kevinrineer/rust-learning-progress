use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo args app")]
struct Args {
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

struct TodoItem {
    id: usize,
    task: String,
    done: bool,
}

struct TodoApp {
    items: Vec<TodoItem>,
    filepath: String,
}

impl TodoItem {
    pub fn display_item(&self) {
        println!("{} {} {}", self.id, self.task, self.done);
    }
}

impl TodoApp {
    fn new() -> Self {
        Self {
            items: vec![],
            filepath: "todos.txt".to_string(),
        }
    }
    fn load_todos(&mut self) {
        let contents = std::fs::read_to_string(&self.filepath).expect("Failed to read the file");
        let todos = &mut self.items;

        for line in contents.lines() {
            let elems: Vec<&str> = line.split('|').collect();
            // If there's stuff, great. Otherwise, it should probably be 0 bytes.
            if elems.len() == 3 {
                let id = elems[0].parse().unwrap_or(0);
                let task = elems[1].to_string();
                let done = elems[2] == "true";
                todos.push(TodoItem { id, task, done });
            }
        }
    }
    pub fn save(self) {
        let filepath = self.filepath;
        for item in &self.items {
            let line = format!("{} {} {} \n", item.id, item.task, item.done);
            std::fs::write(&filepath, line).unwrap();
        }
    }
    pub fn delete_todo(&self, id: usize) {
        /* TODO: Pop id out of vec */
        println!("{} Gone", id);
    }
    pub fn add_todo(&self, description: String) {
        /*TODO:
         * 0. Get id of current end of list and add 1
         * 1. Push new TodoItem to list with id+1, description, and done state of false */
        println!("Added {}", description);
    }
    pub fn mark_todo_done(&self, id: usize) {
        // TODO: Go to id and change done to true
        println!("{} Done", id);
    }
    pub fn list_todos(&self) {
        if self.items.is_empty() {
            println!("No todos!");
        } else {
            for item in &self.items {
                item.display_item();
            }
        }
    }
}

impl Args {
    fn process_subcommand(self, app: &mut TodoApp) {
        match self.command {
            Commands::List => app.list_todos(),
            Commands::Delete { id } => app.delete_todo(id),
            Commands::Add { ref task } => app.add_todo(task.to_string()),
            Commands::Done { id } => app.mark_todo_done(id),
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut app = TodoApp::new();
    app.load_todos();
    args.process_subcommand(&mut app);

    app.save();
}
