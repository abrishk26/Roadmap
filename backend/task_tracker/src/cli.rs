use clap::{Args, Subcommand, ValueEnum};
pub use clap::Parser;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
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
pub struct AddArgs {
    /// Description of the task
    pub description: String,
}

#[derive(Args)]
pub struct ListArgs {
    /// which type of tasks to list
    pub mode: Mode,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// ID of the task to be deleted
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub task_id: u16,
}

#[derive(Args)]
pub struct MarkDoneArgs {
    /// ID of the task to be mark as done
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub task_id: u16,
}

#[derive(Args)]
pub struct MarkInProgressArgs {
    /// ID of the task to be mark as in-progress
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub task_id: u16,
}

#[derive(Args)]
pub struct UpdateArgs {
    /// ID of the task to be updated
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    pub task_id: u16,

    /// Description of the task
    pub description: String,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
