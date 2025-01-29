use sqlx::{query_as, PgPool};
use super::models::*;

pub async fn create_posts(conn: &PgPool, post: &Post) -> Result<Post, sqlx::Error> {
    let result = query_as("INSERT INTO posts(title, content, category, tags, created_at, updated_at) VALUES($1, $2, $3, $4, $5, $6) RETURNING *")
        .bind(&post.title)
        .bind(&post.content)
        .bind(&post.category)
        .bind(&post.tags)
        .bind(&post.created_at)
        .bind(&post.updated_at)
        .fetch_one(conn)
        .await?;
    
    Ok(result)
}

pub async fn select_post(conn: &PgPool, post_id: i32) -> Result<Post, sqlx::Error> {
    let result: Post = query_as("SELECT * FROM posts where id = $1")
        .bind(post_id)
        .fetch_one(conn)
        .await?;
    
    Ok(result)
}

pub async fn select_posts(conn: &PgPool) -> Result<Vec<Post>, sqlx::Error> {
    let result: Vec<Post> = query_as("SELECT * FROM posts")
        .fetch_all(conn)
        .await?;
    
    Ok(result)
}

pub async fn filter_posts(conn: &PgPool, pattern: &str) -> Result<Vec<Post>, sqlx::Error> {
    let result: Vec<Post> = query_as("SELECT * FROM posts")
        .fetch_all(conn)
        .await?;
    let mut response: Vec<Post> = Vec::new();
    for post in result {
        if post.title.as_str().contains(pattern) || post.content.as_str().contains(pattern) || post.category.as_str().contains(pattern) {
            response.push(post);
        }
    }
    Ok(response)
}

pub async fn update_post(conn: &PgPool, post_id: i32, update_info: UpdatePost) -> Result<Post, sqlx::Error> {
    let record: Post = query_as("SELECT * FROM posts WHERE id = $1")
        .bind(post_id)
        .fetch_one(conn)
        .await?;
    
    let title = update_info.title.unwrap_or(record.title);
    let content = update_info.content.unwrap_or(record.content);
    let category = update_info.category.unwrap_or(record.category);
    let tags = update_info.tags.unwrap_or(record.tags);
    
    let result: Post = query_as("UPDATE POSTS SET title = $1, content = $2, category = $3, tags = $4 where id = $5 RETURNING *")
        .bind(title)
        .bind(content)
        .bind(category)
        .bind(tags)
        .bind(post_id)
        .fetch_one(conn)
        .await?;
    
    Ok(result)
}

pub async fn delete_post(conn: &PgPool, post_id: i32) -> Result<Post, sqlx::Error> {
    let record = query_as("DELETE FROM posts WHERE id = $1 RETURNING *")
        .bind(post_id)
        .fetch_one(conn)
        .await?;
    
    Ok(record)
}