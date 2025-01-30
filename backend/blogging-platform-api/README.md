# Blogging Platform API

This is a simple blogging platform API built using Actix-web in Rust.
The API provides endpoints to manage blog posts, including creating, reading, updating, and deleting posts.
It also supports searching for posts based on a pattern in their title, content, category, or tags.
The application uses **PostgreSQL** as the database and **SQLx** for database interactions.

Solution for the [blogging-platform-api](https://roadmap.sh/projects/blogging-platform-api) challenge from [roadmap.sh](https://roadmap.sh).
---

## Features

- **List all posts**: Retrieve a list of all blog posts.
- **Create a post**: Create a new blog post with a title, content, category, and tags.
- **Get a single post**: Retrieve a specific blog post by its ID.
- **Update a post**: Update an existing blog post by its ID.
- **Delete a post**: Delete a blog post by its ID.
- **Search posts**: Search for posts that contain a specific pattern in their title, content, category, or tags.

---

## API Endpoints

### 1. List All Posts

- **GET** `/posts`
- **Description**: Returns a list of all blog posts.
- **Response**: JSON array of posts.

### 2. Create a Post

- **POST** `/posts`
- **Description**: Creates a new blog post.
- **Request Body**:
  ```json
  {
    "title": "Post Title",
    "content": "Post Content",
    "category": "Post Category",
    "tags": ["tag1", "tag2"]
  }
  ```
- **Response**: JSON object representing the created post.

### 3. Get a Single Post

- **GET** `/posts/{post_id}`
- **Description**: Retrieves a specific blog post by its ID.
- **Response**: JSON object representing the post.

### 4. Update a Post

- **PUT** `/posts/{post_id}`
- **Description**: Updates an existing blog post by its ID.
- **Request Body**:
  ```json
  {
    "title": "Updated Title",
    "content": "Updated Content",
    "category": "Updated Category",
    "tags": ["updatedTag1", "updatedTag2"]
  }
  ```
- **Response**: JSON object representing the updated post.

### 5. Delete a Post

- **DELETE** `/posts/{post_id}`
- **Description**: Deletes a blog post by its ID.
- **Response**: HTTP status code indicating success or failure.

### 6. Search Posts

- **GET** `/posts/?term={pattern}`
- **Description**: Searches for posts that contain the specified pattern in their title, content, category, or tags.
- **Response**: JSON array of matching posts.

---

## Getting Started

### Prerequisites

1. **Rust and Cargo**: Ensure you have Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).
2. **PostgreSQL**: Install PostgreSQL on your machine. You can download it from [PostgreSQL's official website](https://www.postgresql.org/download/).
3. **SQLx CLI**: Install the SQLx CLI for database migrations:
   ```bash
   cargo install sqlx-cli
   ```

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/blogging-platform-api.git
   cd blogging-platform-api
   ```

2. Create a `.env` file in the root directory and add the `DATABASE_URL` environment variable:
   ```env
   DATABASE_URL=postgres://username:password@localhost/blog_db
   ```
   Replace `username`, `password`, and `blogging_platform` with your PostgreSQL credentials and database name.

3. Run the database migrations using SQLx CLI:
   ```bash
   sqlx migrate run
   ```

4. Build the project:
   ```bash
   cargo build
   ```

5. Run the server:
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:8080`.

---

## Dependencies

The project uses the following dependencies:

```toml
[dependencies]
actix-web = "4.9.0"
chrono = { version = "0.4.39", features = ["serde"] }
dotenvy = "0.15.7"
env_logger = "0.11.6"
serde = { version = "1.0.217", features = ["derive"] }
sqlx = { version = "0.8.3", features = ["runtime-tokio", "postgres", "derive", "uuid", "chrono"] }
tokio = { version = "1.43.0", features = ["macros", "rt-multi-thread"] }
tower = "0.5.2"
```

---

## Example Requests

### List All Posts

```bash
curl -X GET http://localhost:8080/posts
```

### Create a Post

```bash
curl -X POST http://localhost:8080/posts \
-H "Content-Type: application/json" \
-d '{
  "title": "My First Post",
  "content": "This is the content of my first post.",
  "category": "General",
  "tags": ["blogging", "rust"]
}'
```

### Get a Single Post

```bash
curl -X GET http://localhost:8080/posts/1
```

### Update a Post

```bash
curl -X PUT http://localhost:8080/posts/1 \
-H "Content-Type: application/json" \
-d '{
  "title": "Updated Title",
  "content": "Updated content.",
  "category": "Updated Category",
  "tags": ["updated", "tags"]
}'
```

### Delete a Post

```bash
curl -X DELETE http://localhost:8080/posts/1
```

### Search Posts

```bash
curl -X GET http://localhost:8080/posts/?term=rust
```

---

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or bug fixes.

---

---

## Acknowledgments

- Actix-web team for the excellent web framework.
- SQLx team for the powerful database toolkit.
- Rust community for the amazing ecosystem.

---
