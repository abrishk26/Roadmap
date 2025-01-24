import typer
import lib
from datetime import date
from typing_extensions import Annotated

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
        "ID": len(expenses) + 1, # type: ignore
        "Description": description, # type: ignore
        "Amount": amount, # type: ignore
        "Date": now.strftime("%d-%m-%y"), # type: ignore
    }

    expenses.append(expense)
    lib.write_data(fileName, expenses)

@app.command()
def list():
    """
       list all the expenses.
    """
    print("Listing expenses")

@app.command()
def summary(month: int):
    """
        summarize the expense by calculating the total amount.
    """
    print("Printing Summary")

@app.command()
def delete(
    id: int,
    force: Annotated[
        bool, typer.Option(prompt="Are you sure you want to delete the expense record?")
    ],):
    """
        delete the expense with the given ID.
    """
    print("User deleted successfully")

if __name__ == "__main__":
    app()

