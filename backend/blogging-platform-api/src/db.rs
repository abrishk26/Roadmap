use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::error::Error;
use dotenvy::dotenv;

pub async fn establish_connection() -> Result<PgPool, Box<dyn Error>> {
    dotenv()?;
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url).await?;
    return Ok(pool);
}