mod utils;
mod cli;


use cli::*;
use utils::*;

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
            list_tasks(&list_args);
        }
        Commands::MarkDone(mark_done_args) => {
            mark_done(&mark_done_args);
        }
        Commands::MarkInProgress(mark_in_progress_args) => {
            mark_in_progress(&mark_in_progress_args);
        }
        Commands::Update(update_args) => {
            update_task(&update_args);
        }
    }
}

