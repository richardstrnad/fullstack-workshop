use axum::{
    routing::{delete, get, post},
    Router,
};
use controllers::{add_item, create_shopping_list, delete_item, get_items};
use tower_http::cors::CorsLayer;

use database::InMemoryDatabase;
use std::sync::{Arc, RwLock};

mod database;
type Database = Arc<RwLock<InMemoryDatabase>>;

mod controllers;

#[tokio::main]
async fn main() {
    let db = Database::default();
    let app = Router::new()
        .route("/list/:list_uuid/items", get(get_items).post(add_item))
        .route("/list/:list_uuid/items/:item_uuid", delete(delete_item))
        .route("/list", get(create_shopping_list))
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
