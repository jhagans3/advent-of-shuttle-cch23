use axum::http::{StatusCode, Uri};

/// # Task 1: Everything is OK
/// Deploy a minimal working web app to your CCH23 Shuttle project.
///
/// The root endpoint / should respond with a 200 OK status code to GET requests.
/// Responding with a "Hello, world!" string, like the starter templates do,
/// is enough to accomplish this.
pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

/// # Task 2: Fake error (0 bonus points)
/// For this bonus task, add an endpoint on /-1/error that responds
/// with the status code 500 Internal Server Error to GET requests.
/// The response body content does not matter.
pub async fn error(uri: Uri) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error for path: {}", uri.path()),
    )
}
