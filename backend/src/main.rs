use axum::{
    routing::{delete, get, post},
    Router,
};
use controllers::{add_item, delete_item, get_items};
use tower_http::cors::CorsLayer;

use database::InMemoryDatabase;
use std::sync::{Arc, RwLock};

mod database;
type Database = Arc<RwLock<InMemoryDatabase>>;

mod controllers;

// #[derive(Deserialize, Serialize)]
// struct Workshop {
//     attendees_count: i32,
//     people_like_it: bool,
// }

// async fn hello_world() -> impl IntoResponse {
//     "Hello World"
// }
//
// async fn hello_name(Path(name): Path<String>) -> impl IntoResponse {
//     format!("Hello {name}")
// }
//
// async fn workshop_echo(Json(workshop): Json<Workshop>) -> impl IntoResponse {
//     Json(workshop)
// }

#[tokio::main]
async fn main() {
    let db = Database::default();
    let app = Router::new()
        .route("/items", get(get_items).post(add_item))
        .route("/items/:uuid", delete(delete_item))
        .layer(CorsLayer::permissive())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
