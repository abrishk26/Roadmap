# Task Tracker

Task Tracker is a command-line application to manage tasks. It allows you to add, list, update, delete, and mark tasks as done or in-progress.

## Features

- Add a new task
- List all tasks or filter by status (all, done, todo, in-progress)
- Update a task's description
- Delete a task
- Mark a task as done
- Mark a task as in-progress

## Installation

1. Clone the repository:
    ```sh
    git clone <repository-url>
    cd task_tracker
    ```

2. Build the project using Cargo:
    ```sh
    cargo build
    ```

## Usage

Run the application using Cargo:
```sh
cargo run -- <command> [arguments]

Commands
add <description>: Add a new task with the given description.
list <mode>: List tasks based on the mode (all, done, todo, in-progress).
update <task_id> <description>: Update the task with the specified ID.
delete <task_id>: Delete the task with the specified ID.
mark-done <task_id>: Mark the task with the specified ID as done.
mark-in-progress <task_id>: Mark the task with the specified ID as in-progress.
Examples
Add a new task:
cargo run -- add "Finish the report"
List all tasks:
cargo run -- list all
Update a task:
cargo run -- update 1 "Finish the report and send it"
Delete a task:
cargo run -- delete 1
Mark a task as done:
cargo run -- mark-done 1
Mark a task as in-progress:
cargo run -- mark-in-progress 1
