mod db;
mod models;
mod post_repository;

pub use {db::*, models::*, post_repository::*};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[sqlx::test]
    async fn test_create_post(pool: sqlx::PgPool) -> sqlx::Result<()> {
        
        let post = Post {
            id: 0,
            title: "Test title".to_string(),
            content: "Test Content".to_string(),
            category: "Test category".to_string(),
            tags: vec!["Tech".to_string(), "Programming".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(), 
        };
        
        let result = create_post(&pool, &post).await?;
        assert_eq!(post.title, result.title);
        
        let result = select_post(&pool, 1).await?;
        assert_eq!(result.id, 1);
        
        let result = select_posts(&pool).await?;
        assert_eq!(result.len(), 1);
        
        let result = filter_posts(&pool, "demo").await?;
        assert_eq!(result.len(), 0);
        
        let result = filter_posts(&pool, "Test").await?;
        assert_eq!(result.len(), 1);
        
        let updated_post = UpdatePost {
            title: Some(String::from("Update title")),
            content: Some(String::from("Update content")),
            category: None,
            tags: None
        };
        
        let result = update_post(&pool, 1, updated_post).await?;
        assert_eq!(result.title, String::from("Update title"));
        assert_eq!(result.tags, vec!["Tech".to_string(), "Programming".to_string()]);
        
        let result = delete_post(&pool, 1).await?;
        assert_eq!(result.id, 1);
        assert_eq!(result.title, String::from("Update title"));
        
        Ok(())
    }
    
}