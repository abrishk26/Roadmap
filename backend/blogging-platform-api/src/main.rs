mod handlers;
use actix_web::{middleware::Logger, App, HttpServer, web};
use env_logger::Env;
use sqlx::PgPool;
use db_helper::establish_connection;

#[derive(Clone)]
pub struct AppState {
    pub conn: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let app_state = web::Data::new(AppState {
        conn: establish_connection().await.unwrap()
    });
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(app_state.clone())
            .service(handlers::create)
            .service(handlers::get_post)
            .service(handlers::get_posts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
