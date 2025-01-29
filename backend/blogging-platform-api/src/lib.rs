use sqlx::PgPool;

mod db;
mod models;
mod post_repository;

pub use {db::*, models::*, post_repository::*};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[sqlx::test]
    async fn test_create_post(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;
        
        let post = Post {
            id: 0,
            title: "Test title".to_string(),
            content: "Test Content".to_string(),
            category: "Test category".to_string(),
            tags: vec!["Tech".to_string(), "Programming".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(), 
        };
        
        let result = create_posts(&mut conn, &post).await?;
        
        assert_eq!(post.title, result.title);
        
        Ok(())
    }
    
}