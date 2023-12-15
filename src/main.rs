use axum::{routing::get, Router};

pub mod handlers;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(handlers::neg_one::hello_world))
        .route("/-1/error", get(handlers::neg_one::error))
        // .route("/1/:num1/:num2", get(cube_the_bits));
        .route("/1/*ids", get(handlers::day_01::sled_id_system));

    Ok(router.into())
}
