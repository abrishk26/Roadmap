use clap::{Parser, Args, Subcommand, ValueEnum};
// A struct that represent the task.
#[derive(serde::Deserialize, serde::Serialize)]
struct Task {
    id: i32,
    description: String,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>
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
    InProgress
}


#[derive(Args)]
struct AddArgs {
    /// Description of the task
    description: String,
}

#[derive(Args)]
struct ListArgs {
    /// which type of tasks to list
    mode: Mode
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
    description: String
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
#[command(name="task_tracker")]
#[command(version="1.0")]
#[command(about = "Cli app to manage task", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

}



fn main() {
    let _cli = Cli::parse();
}
