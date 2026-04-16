use std::sync::Arc;

use axum::{Router, routing::get};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::error::ApiError;
use crate::response::{ApiResponse, Message};
use crate::{error::my_function};
use crate::rounters::init_user_router;

mod response;
mod error;
mod rounters;
mod db;

fn init_router() -> Router<Arc<AppState>> {
    let user_router = init_user_router();
    Router::new()
        .route("/", get(get_user))
        .route("/api/v1", get(my_function))
        .merge(user_router)
}

pub struct AppState {
    pub db: PgPool
}

async fn get_user() -> Result<ApiResponse, ApiError> {
    Ok(ApiResponse::JsonData(vec![Message { message: "hello".to_string() }]))
}

#[tokio::main]
async fn main() {

    dotenvy::dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str).await
        .expect("can't connect to database");

    let app_state = Arc::new(AppState { db: pool });

    let router: Router<()> = init_router().with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
