use actix_web::{post, get, web, HttpResponse};

use super::AppState;
use db_helper::*;

#[derive(serde::Deserialize)]
struct CreatePost {
    title: String,
    content: String,
    category: String,
    tags: Vec<String>,
}

#[post("/posts")]
pub async fn create(state: web::Data<AppState>, payload: web::Json<CreatePost>) -> HttpResponse {
    let post = Post {
        id: 0,
        title: payload.title.clone(),
        content: payload.content.clone(),
        category: payload.category.clone(),
        tags: payload.tags.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let result = create_post(&state.conn, &post).await;

    match result {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(err) => {
            // Handle specific errors
            match err {
                sqlx::Error::Database(db_err) => {
                    // Check for unique constraint violations or other database errors
                    if db_err.message().contains("unique constraint") {
                        HttpResponse::BadRequest().body("Duplicate entry: title must be unique")
                    } else {
                        HttpResponse::InternalServerError().body("Internal server error")
                    }
                }
                _ => HttpResponse::InternalServerError().body("Internal server error"),
            }
        }
    }
}

#[get("posts/{post_id}")]
pub async fn get_post(state: web::Data<AppState>, post_id: web::Path<i32>) -> HttpResponse {
    let result = select_post(&state.conn, post_id.into_inner()).await;
    match result {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(err) => {
            // Handle specific errors
            match err {
                sqlx::Error::Database(db_err) => {
                    // Check for unique constraint violations or other database errors
                    if db_err.message().contains("unique constraint") {
                        HttpResponse::BadRequest().body("Duplicate entry: title must be unique")
                    } else {
                        HttpResponse::InternalServerError().body("Internal server error")
                    }
                }
                _ => HttpResponse::InternalServerError().body("Internal server error"),
            }
        }
    } 
}

#[get("/posts")]
pub async fn get_posts(state: web::Data<AppState>) -> HttpResponse {
    let result = select_posts(&state.conn).await;
    
    match result {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(err) => {
            // Handle specific errors
            match err {
                sqlx::Error::Database(db_err) => {
                    // Check for unique constraint violations or other database errors
                    if db_err.message().contains("unique constraint") {
                        HttpResponse::BadRequest().body("Duplicate entry: title must be unique")
                    } else {
                        HttpResponse::InternalServerError().body("Internal server error")
                    }
                }
                _ => HttpResponse::InternalServerError().body("Internal server error"),
            }
        }
    }  
}
