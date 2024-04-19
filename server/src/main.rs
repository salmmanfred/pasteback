#[macro_use]
extern crate lazy_static;
use axum::{ http::StatusCode, response::Html, routing::{get, post}, Router};
mod image_hand;
mod paste_hand;
#[tokio::main]
async fn main() {
    println!("Starting server");
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/favicon", get(image_hand::image))
        .route("/new", post(paste_hand::post_token))
        .route("/ret/:token", get(paste_hand::ret_token))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn health()->StatusCode{
    StatusCode::OK
}

async fn root() -> Html<String>{
    Html(include_str!("../index.html").to_string())
}

