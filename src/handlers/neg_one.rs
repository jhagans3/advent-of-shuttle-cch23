use axum::http::{StatusCode, Uri};

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn error(uri: Uri) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error for path: {}", uri.path()),
    )
}
