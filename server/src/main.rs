#[macro_use]
extern crate lazy_static;
use axum::{ http::StatusCode, response::Html, routing::{get, post}, Router};
mod image_hand;
mod paste_hand;
#[tokio::main]
async fn main() {
    println!("Starting server");

    openfile::write_file_bytes("./favicon.ico", include_bytes!("../img/favicon.ico").to_vec()).unwrap();
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/favicon", get(image_hand::image))
        .route("/new", post(paste_hand::post_token))
        .route("/ret/:token", get(paste_hand::ret_token))
        .route("/health", get(health));

    let port = option_env!("PORT").unwrap(); //"3000";//
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}",port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn health()->StatusCode{
    StatusCode::OK
}

async fn root() -> Html<String>{
    Html(include_str!("../index.html").to_string())
}

