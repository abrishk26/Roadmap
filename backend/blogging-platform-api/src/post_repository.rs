use std::error::Error;
use sqlx::Row;
use super::models::*;
use sqlx::PgConnection;
use sqlx::query;

pub async fn create_posts(conn: &mut PgConnection, post: &Post) -> Result<Post, Box<dyn Error>> {
    let result = query("INSERT INTO posts VALUES($1, $2, $3, $4, $5, $6) RETURNING *")
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