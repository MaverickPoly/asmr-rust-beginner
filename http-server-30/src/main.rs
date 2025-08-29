use axum::{Router, routing::get, Json};
use serde_json::{json, Value};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u32,
    username: String,
    email: String
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/user", get(get_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello, world!"
}

async fn get_user() -> Json<Value> {
    let user = User {
        id: 1,
        username: String::from("mostafa"),
        email: String::from("mostafa@gmail.com")
    };

    Json(json!({"user": user}))
}
