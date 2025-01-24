import typer
import lib
from datetime import date
from typing_extensions import Annotated
from prettytable import PrettyTable
from typing import Optional

months = {
    1: "January",
    2: "February",
    3: "March",
    4: "April",
    5: "May",
    6: "June",
    7: "July",
    8: "August",
    9: "September",
    10: "October",
    11: "November",
    12: "December"
}


table = PrettyTable()
table.field_names = ["ID", "Description", "Amoount", "Date"]

app = typer.Typer(no_args_is_help=True)
fileName = "./tasks.json"
@app.command()
def add(
    description: str, 
    amount: Annotated[int, typer.Argument(min=1)]):
    """
        add the expense with the given DESCRIPTION and AMOUNT.
    """
    now = date.today()
    expenses = lib.read_data(fileName)
    expense = {
        "ID": int(expenses[len(expenses) - 1]["ID"]) + 1, # type: ignore
        "Description": description, # type: ignore
        "Amount": amount, # type: ignore
        "Date": now.strftime("%d-%m-%y"), # type: ignore
    }

    expenses.append(expense)
    lib.write_data(fileName, expenses)

    print(f"Expense added successfully (ID: {expense["ID"]})")

@app.command()
def list():
    """
       list all the expenses.
    """
    expenses = lib.read_data(fileName)

    for expense in expenses:
        table.add_row([expense["ID"], expense["Description"], expense["Amount"], expense["Date"]])

    print(table)

@app.command()
def summary(month: Annotated[Optional[int], typer.Argument()] = None):
    """
        summarize the expense by calculating the total amount.
    """
    totalExpense = 0
    expenses = lib.read_data(fileName)
    if month != None:
        for expense in expenses:
            if int(expense["Date"].split("-")[1]) == month:
                totalExpense += expense["Amount"]
        print(f"Total expenses for {months[month]}: {totalExpense}")
        return
       
    for expense in expenses:
        totalExpense += expense["Amount"]

    print("Total expense:", totalExpense)

@app.command()
def delete(
    id: int,
    force: Annotated[
        bool, typer.Option(prompt="Are you sure you want to delete the expense record?")
    ],):
    """
        delete the expense with the given ID.
    """
    if force:
        print("Deleting expense")
    else:
        print("Operation cancelled")
    expenses = lib.read_data(fileName)

    for i, expense in enumerate(expenses):
        if expense["ID"] == id:
            expenses.pop(i)
            lib.write_data(fileName, expenses)
            print("Deleting completed")
            return

    print("The expense with the specified ID does not exist")

if __name__ == "__main__":
    app()

