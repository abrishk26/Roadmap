# Expense Tracker CLI

This is a **command-line interface (CLI) application** built using Python and Typer for managing and tracking expenses. The application allows users to add, list, summarize, and delete expenses stored in a JSON file.


## Demo

Click the thumbnail below to view a demo of the Expense Tracker CLI:

[![asciicast](https://asciinema.org/a/ZcB5fOj1Du8v3f3JH2NdeheLI.svg)](https://asciinema.org/a/ZcB5fOj1Du8v3f3JH2NdeheLI)

## Features

* **Add Expense** : Add an expense by providing a description and amount.
* **List Expenses** : Display all the recorded expenses in a tabular format.
* **Summarize Expenses** : Calculate the total expenses for a specific month or all months.
* **Delete Expense** : Remove an expense by its unique ID.

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-folder>
   ```
2. Install dependecies:
   ```bash
   uv install
   ```

## Usage

Run the CLI application:

```bash
uv run <script_name>.py
```

### Commands

#### 1. Add Expense

Add a new expense to the tracker.

```bash
uv run <script_name>.py add <description> <amount>
```

**Example:**

```bash
uv run <script_name>.py add "Lunch" 50
```

#### 2. List Expenses

List all the recorded expenses in a table format.

```bash
uv run <script_name>.py list
```

#### 3. Summarize Expenses

Calculate the total expenses for all months or a specific month.

* All months:
  ```bash
  uv run <script_name>.py summary
  ```
* Specific month (e.g., January):
  ```bash
  uv run <script_name>.py summary 1
  ```

#### 4. Delete Expense

Delete an expense by its unique ID.

```bash
uv run <script_name>.py delete <id>
```

You will be prompted for confirmation unless the `--force` flag is used.

**Example:**

```bash
uv run <script_name>.py delete 1
```

## File Structure

```
project-folder/
├── main.py         # Main application script
├── lib.py                   # Helper library for reading/writing JSON data
├── tasks.json               # JSON file for storing expense data
├── pyproject.toml         # Python dependencies
└── README.md                # Project documentation
```

## Dependencies

* **Typer** : For creating CLI commands.
* **PrettyTable** : For displaying expenses in a table format.
* **Typing Extensions** : For enhanced type annotations.

Install all dependencies with:

```bash
uv install
```

## JSON Storage Structure

Each expense is stored as a JSON object in `tasks.json`. Example:

```json
[
  {
    "ID": 1,
    "Description": "Lunch",
    "Amount": 50,
    "Date": "24-01-25"
  }
]
```

## Future Enhancements

* Add support for exporting expenses to CSV or Excel.
* Implement search functionality for expenses by description.
* Add a feature to edit an existing expense.

---

**Happy Expense Tracking!**
