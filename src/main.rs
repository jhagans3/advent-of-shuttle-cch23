use axum::{
    extract::Path,
    http::{StatusCode, Uri},
    routing::get,
    Router,
};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn neg_one_error(uri: Uri) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error for path: {}", uri.path()),
    )
}

async fn cube_the_bits(Path((num1, num2)): Path<(i32, i32)>) -> (StatusCode, String) {
    let xor = num1 ^ num2;
    let pow = i32::pow(xor, 3);

    (StatusCode::OK, format!("{pow}"))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(neg_one_error))
        .route("/1/:num1/:num2", get(cube_the_bits));

    Ok(router.into())
}
