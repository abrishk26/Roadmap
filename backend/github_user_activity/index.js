const { Table } = require("console-table-printer");

const response = await fetch("https://api.github.com/users/abrishk26/events", {
    method: "GET",
    headers: {"User-Agent": "abrishk26"}
});

const events = await response.json();
const t = new Table({
    columns: [
        { name: "ID", alignment: "left", color: "blue" },
        { name: "Event Type", alignment: "left" },
        { name: "Repo", alignment: "left"}
    ]
});

for (let e of events) {
    t.addRow({ ID: e["id"], "Event Type": e["type"], Repo: e["repo"]["name"]});
}

t.printTable();