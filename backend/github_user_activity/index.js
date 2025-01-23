const { Table } = require("console-table-printer");

const t = new Table({
    columns: [
        { name: "ID", alignment: "left", color: "blue" },
        { name: "Event Type", alignment: "left" },
        { name: "Repo", alignment: "left" }
    ]
});

if (typeof Bun.argv[2] === "undefined") {
    console.log("Please provide the user name and try again.");
} else {
    const username = Bun.argv[2];

    const response = await fetch(`https://api.github.com/users/${username}/events`, {
        method: "GET",
        headers: { "User-Agent": "abrishk26" }
    });

    const events = await response.json();

    for (let e of events) {
        t.addRow({ ID: e["id"], "Event Type": e["type"], Repo: e["repo"]["name"] });
    }

    t.printTable();
}
