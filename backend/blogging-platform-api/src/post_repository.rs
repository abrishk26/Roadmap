use sqlx::query_as;
use sqlx::Row;
use super::models::*;
use sqlx::PgConnection;
use sqlx::query;

pub async fn create_posts(conn: &mut PgConnection, post: &Post) -> Result<Post, sqlx::Error> {
    let result = query("INSERT INTO posts(title, content, category, tags, created_at, updated_at) VALUES($1, $2, $3, $4, $5, $6) RETURNING *")
        .bind(&post.title)
        .bind(&post.content)
        .bind(&post.category)
        .bind(&post.tags)
        .bind(&post.created_at)
        .bind(&post.updated_at)
        .fetch_one(conn)
        .await?;
    
    Ok(Post {
        id: result.get("id"),
        title: result.get("title"),
        content: result.get("content"),
        category: result.get("category"),
        tags: result.get("tags"),
        created_at: result.get("created_at"),
        updated_at: result.get("updated_at")
    })
}

pub async fn select_post(conn: &mut PgConnection, post_id: i32) -> Result<Post, sqlx::Error> {
    let result = query("SELECT * FROM posts where id = $1")
        .bind(post_id)
        .fetch_one(conn)
        .await?;
    
    Ok(Post {
        id: result.get("id"),
        title: result.get("title"),
        content: result.get("content"),
        category: result.get("category"),
        tags: result.get("tags"),
        created_at: result.get("created_at"),
        updated_at: result.get("updated_at")
    })
}

pub async fn select_posts(conn: &mut PgConnection) -> Result<Vec<Post>, sqlx::Error> {
    let result: Vec<Post> = query_as("SELECT * FROM posts")
        .fetch_all(conn)
        .await?;
    
    Ok(result)
}