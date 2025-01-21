use clap::{Args, Parser, Subcommand, ValueEnum};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};
// A struct that represent the task.
#[derive(serde::Deserialize, serde::Serialize)]
struct Task {
    id: u16,
    description: String,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n\tID: {}\n\tDescription: {}\n\tStatus: {}\n\tCreated_At: {}\n\tUpdated_At: {}\n}}",
            self.id, self.description, self.status, self.created_at, self.updated_at
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// List all tasks
    All,
    /// List task that are Done
    Done,
    /// List tasks that we haven't yet begin doing
    Todo,
    /// List task that are in-progress
    InProgress,
}

#[derive(Args)]
struct AddArgs {
    /// Description of the task
    description: String,
}

#[derive(Args)]
struct ListArgs {
    /// which type of tasks to list
    mode: Mode,
}

#[derive(Args)]
struct DeleteArgs {
    /// ID of the task to be deleted
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    task_id: u16,
}

#[derive(Args)]
struct MarkDoneArgs {
    /// ID of the task to be mark as done
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    task_id: u16,
}

#[derive(Args)]
struct MarkInProgressArgs {
    /// ID of the task to be mark as in-progress
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    task_id: u16,
}

#[derive(Args)]
struct UpdateArgs {
    /// ID of the task to be updated
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    task_id: u16,

    /// Description of the task
    description: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Add task to the task list
    Add(AddArgs),

    /// List all the tasks that exists.
    List(ListArgs),

    /// Delete the task with the specified task_id
    Delete(DeleteArgs),

    /// Mark the task with the specified task_id as done
    MarkDone(MarkDoneArgs),

    /// Mark the task with the specified task_id as in-progress
    MarkInProgress(MarkInProgressArgs),

    /// Update the task with the specified task_id with the given description
    Update(UpdateArgs),
}

#[derive(Parser)]
#[command(name = "task_tracker")]
#[command(version = "1.0")]
#[command(about = "Cli app to manage task", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(add_args) => {
            add_task(&add_args.description);
        }
        Commands::Delete(delete_args) => {
            delete_task(delete_args.task_id);
        }
        Commands::List(list_args) => {
            let tasks = read_tasks();
            if tasks.len() == 0 {
                println!("Empty");
            } else if list_args.mode == Mode::All {
                for task in tasks {
                    println!("{task}");
                }
            } else if list_args.mode == Mode::Done {
                for task in tasks {
                    if task.status == "done" {
                        println!("{task}");
                    } else {
                        println!("Empty");
                    }
                }
            } else if list_args.mode == Mode::InProgress {
                for task in tasks {
                    if task.status == "in-progress" {
                        println!("{task}");
                    } else {
                        println!("Empty");
                    }
                }
            } else if list_args.mode == Mode::Todo {
                for task in tasks {
                    if task.status == "todo" {
                        println!("{task}");
                    } else {
                        println!("Empty");
                    }
                }
            }
        }
        Commands::MarkDone(mark_done_args) => {
            let mut tasks = read_tasks();

            let index = tasks
                .iter()
                .position(|n| n.id == mark_done_args.task_id)
                .expect("Task with the specified id does not exits");

            tasks[index].status = "done".to_string();

            write_tasks(tasks);
            println!("marked successfully");
        }
        Commands::MarkInProgress(mark_in_progress_args) => {
            let mut tasks = read_tasks();

            let index = tasks
                .iter()
                .position(|n| n.id == mark_in_progress_args.task_id)
                .expect("Task with the specified id does not extist");

            tasks[index].status = "in-progress".to_string();
            write_tasks(tasks);
            println!("marked sucessfully");
        }
        Commands::Update(update_args) => {
            let mut tasks = read_tasks();

            let index = tasks
                .iter()
                .position(|n| n.id == update_args.task_id)
                .expect("Task with the specified id does not extist");

            tasks[index].description = update_args.description.clone();
            write_tasks(tasks);
            println!("updated sucessfully");
        }
    }
}

fn add_task(description: &str) {
    let mut tasks = read_tasks();

    let task = Task {
        id: tasks.len() as u16 + 1,
        description: description.to_string(),
        status: "todo".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    tasks.push(task);

    write_tasks(tasks);
}

fn delete_task(task_id: u16) {
    let mut tasks = read_tasks();

    let index = tasks
        .iter()
        .position(|n| n.id == task_id)
        .expect("The element with the specified id does not exist");

    let _ = tasks.remove(index);

    write_tasks(tasks);
}

fn read_tasks() -> Vec<Task> {
    let mut content = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("tasks.json")
        .expect("Unable to open file.");
    let tasks: Vec<Task>;

    file.read_to_string(&mut content)
        .expect("Unable to read file");

    if content.trim().len() == 0 {
        tasks = Vec::new();
    } else {
        tasks = serde_json::from_str(&content).expect("Unable to convert tasks");
    }

    tasks
}

fn write_tasks(tasks: Vec<Task>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.json")
        .expect("Unable to open file.");
    let content = serde_json::to_string_pretty(&tasks).expect("Unable to convert into json");

    file.write_all(&content.as_bytes())
        .expect("Unable to write to a file");
}
