pub use chrono::prelude::*;

pub struct Post {
    id: Option<i32>,
    title: String,
    content: String,
    category: String,
    tags: Vec<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}