use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Workshop {
    attendees_count: i32,
    people_like_it: bool,
}

async fn hello_world() -> impl IntoResponse {
    "Hello World"
}

async fn hello_name(Path(name): Path<String>) -> impl IntoResponse {
    format!("Hello {name}")
}

async fn workshop_echo(Json(workshop): Json<Workshop>) -> impl IntoResponse {
    Json(workshop)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/:name", get(hello_name))
        .route("/your-route", post(workshop_echo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
