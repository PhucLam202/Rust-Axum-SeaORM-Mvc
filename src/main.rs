use crate::routers::user_router::user_router;
use axum::{Extension, Router};
use sea_orm::Database;
use server::postgres_server::conn_db;
use std::sync::Arc;
mod controllers;
mod helpers;
mod middleware;
mod models;
mod routers;
mod server;
#[tokio::main]
async fn main() {
    let postgres_url = conn_db().await;
    let db = match Database::connect(&postgres_url).await {
        Ok(db) => db,
        Err(err) => {
            println!("Failed to connect to PostgreSQL: {}", err);
            std::process::exit(1);
        }
    };
    let db = Arc::new(db);
    let user_routes = user_router(Extension(db.clone()));
    let app = Router::new()
        .merge(user_routes)
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
