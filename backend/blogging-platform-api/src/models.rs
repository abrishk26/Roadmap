pub use chrono::prelude::*;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>
}