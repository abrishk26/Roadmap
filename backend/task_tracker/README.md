# Task Tracker
Sample solution for the [task-tracker](https://roadmap.sh/projects/task-tracker) challenge from [roadmap.sh](https://roadmap.sh).

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
    cargo build --release
    ```
3. Go to the directory where the binary **task_tracker** is located
   ```sh
   cd ./target/release
   ```

## Usage

Run the application using Cargo:
```sh
task_tracker <command> [arguments]
```

## Commands
```sh
add <description>: Add a new task with the given description.
list <mode>: List tasks based on the mode (all, done, todo, in-progress).
update <task_id> <description>: Update the task with the specified ID.
delete <task_id>: Delete the task with the specified ID.
mark-done <task_id>: Mark the task with the specified ID as done.
mark-in-progress <task_id>: Mark the task with the specified ID as in-progress.
```

## Examples
Add a new task:
```sh
task_tracker add "Finish the report"
```
List all tasks:
```sh
task_tracker list all
```
Update a task:
```sh
task_tracker update 1 "Finish the report and send it"
```
Delete a task:
```sh
task_tracker delete 1
```
Mark a task as done:
```sh
task_tracker mark-done 1
```
Mark a task as in-progress:
```sh
task_tracker mark-in-progress 1
```
