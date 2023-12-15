use axum::{extract::Path, http::StatusCode};

/// # Task 1: Cube the bits
/// Santa needs your programming expertise to recalibrate the packet IDs
/// in his packet management system.
///
/// Implement a GET endpoint `/1/<num1>/<num2>` that takes 2 integers in the path,
/// `num1` and `num2`, and returns the result of `(num1 XOR num2) POW 3`,
/// where XOR is the exclusive OR operation, and POW is exponentiation.
pub async fn cube_the_bits(Path((num1, num2)): Path<(i32, i32)>) -> (StatusCode, String) {
    let xor = num1 ^ num2;
    let pow = i32::pow(xor, 3);

    (StatusCode::OK, format!("{pow}"))
}

/// # Task 2: The sled ID system (100 bonus points)
/// After the packet IDs are calibrated and the packets are packed into sleds,
/// Santa needs to calibrate the sled ID.
///
/// The formula is very similar: All packet IDs (integers) are XOR'ed with each other,
/// and then the result is (again) raised to the power of 3. The catch is that there
/// can be between 1 and 20 packets in a sled!
///
/// Adapt the endpoint from Task 1 so that it can also be used to calculate sled IDs.
pub async fn sled_id_system(Path(path): Path<String>) -> (StatusCode, String) {
    let str_vec: Vec<&str> = (path.as_str()).split('/').collect();
    let ids = str_vec.iter().filter_map(|x| x.parse::<i32>().ok());
    let xor = ids.into_iter().reduce(|acc, e| acc ^ e).unwrap();
    let pow = i32::pow(xor, 3);

    (StatusCode::OK, format!("{pow}"))
}
