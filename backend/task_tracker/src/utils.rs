use crate::cli::*;
use std::fs::OpenOptions;
use std::io::{Read, Write};

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

pub fn add_task(description: &str) {
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

pub fn delete_task(task_id: u16) {
    let mut tasks = read_tasks();

    let index = tasks
        .iter()
        .position(|n| n.id == task_id)
        .expect("The element with the specified id does not exist");

    let _ = tasks.remove(index);

    write_tasks(tasks);
}

pub fn list_tasks(list_args: &ListArgs) {
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

pub fn mark_done(mark_done_args: &MarkDoneArgs) {
    let mut tasks = read_tasks();

            let index = tasks
                .iter()
                .position(|n| n.id == mark_done_args.task_id)
                .expect("Task with the specified id does not exits");

            tasks[index].status = "done".to_string();

            write_tasks(tasks);
            println!("marked successfully");
}

pub fn mark_in_progress(mark_in_progress_args: &MarkInProgressArgs) {
    let mut tasks = read_tasks();

            let index = tasks
                .iter()
                .position(|n| n.id == mark_in_progress_args.task_id)
                .expect("Task with the specified id does not extist");

            tasks[index].status = "in-progress".to_string();
            write_tasks(tasks);
            println!("marked sucessfully");
}

pub fn update_task(update_args: &UpdateArgs) {
    let mut tasks = read_tasks();

            let index = tasks
                .iter()
                .position(|n| n.id == update_args.task_id)
                .expect("Task with the specified id does not extist");

            tasks[index].description = update_args.description.clone();
            write_tasks(tasks);
            println!("updated sucessfully");
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
