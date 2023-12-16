use axum::{
    routing::{get, post},
    Router,
};

pub mod handlers;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(handlers::neg_one::hello_world))
        .route("/-1/error", get(handlers::neg_one::error))
        // .route("/1/:num1/:num2", get(cube_the_bits));
        .route("/1/*ids", get(handlers::day_01::sled_id_system))
        .route("/4/strength", post(handlers::day_04::reindeer_cheer))
        .route("/4/contest", post(handlers::day_04::candy_contest));

    Ok(router.into())
}
