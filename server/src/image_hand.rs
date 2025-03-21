use axum::response::{IntoResponse, Response};

pub async fn favicon() -> Response {
    include_bytes!("../favicon.ico").into_response()
}
