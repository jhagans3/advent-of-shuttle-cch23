use axum::{extract::Path, http::StatusCode};

async fn cube_the_bits(Path((num1, num2)): Path<(i32, i32)>) -> (StatusCode, String) {
    let xor = num1 ^ num2;
    let pow = i32::pow(xor, 3);

    (StatusCode::OK, format!("{pow}"))
}

pub async fn sled_id_system(Path(path): Path<String>) -> (StatusCode, String) {
    let str_vec: Vec<&str> = (path.as_str()).split('/').collect();
    let ids = str_vec.iter().filter_map(|x| x.parse::<i32>().ok());
    let xor = ids.into_iter().reduce(|acc, e| acc ^ e).unwrap();
    let pow = i32::pow(xor, 3);

    (StatusCode::OK, format!("{pow}"))
}
