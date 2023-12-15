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

// async fn cube_the_bits(Path((num1, num2)): Path<(i32, i32)>) -> (StatusCode, String) {
//     let xor = num1 ^ num2;
//     let pow = i32::pow(xor, 3);

//     (StatusCode::OK, format!("{pow}"))
// }

async fn sled_id_system(Path(path): Path<String>) -> (StatusCode, String) {
    let str_vec: Vec<&str> = (path.as_str()).split('/').collect();
    let ids = str_vec.iter().filter_map(|x| x.parse::<i32>().ok());
    let xor = ids.into_iter().reduce(|acc, e| acc ^ e).unwrap();
    let pow = i32::pow(xor, 3);

    (StatusCode::OK, format!("{pow}"))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(neg_one_error))
        // .route("/1/:num1/:num2", get(cube_the_bits));
        .route("/1/*ids", get(sled_id_system));

    Ok(router.into())
}
