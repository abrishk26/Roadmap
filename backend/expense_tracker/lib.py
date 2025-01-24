import os
import json

def write_data(filename, data):
    with open(filename, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)

def read_data(filename):
    if os.path.exists(filename):
        with open(filename, "r", encoding="utf-8") as f:
            content = json.load(f)
            return content
    else:
        return []