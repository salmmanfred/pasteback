

use axum::{body::Body, http::{header, StatusCode}, response::IntoResponse};

use tokio_util::io::ReaderStream;

pub async fn image() -> impl IntoResponse {
    let file = match tokio::fs::File::open("./favicon.ico").await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
   
    // 

    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    //let body = BodyDataStream::new(stream);
    let body = Body::from_stream(stream);
    //let body = into_data_stream(stream);
    let headers = [
        (header::CONTENT_TYPE, "image"),
        (
            header::CONTENT_DISPOSITION,
            "attachment; filename=\"favicon.ico\"",
        ),
    ];

    Ok((headers, body))
}