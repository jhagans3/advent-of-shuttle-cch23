use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Counter {
    elf: i32,
}

/// # Task 1: Never count on an elf
/// Elves are notorious for their hide-and-seek skills, and now they've
/// hidden themselves in strings of text!
/// Create an endpoint /6 that takes a POST request with a raw string as
/// input and count how many times the substring "elf" appears.
/// The output should be a JSON object containing the count of the string "elf".
pub async fn elf_count(body: String) -> (StatusCode, Json<Counter>) {
    let count = body.as_str().matches("elf").count();
    (StatusCode::OK, Json(Counter { elf: count as i32 }))
}
