CREATE TABLE IF NOT EXISTS tasks (
    task_id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL, -- Added user_id column
    title TEXT UNIQUE NOT NULL,
    description TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);