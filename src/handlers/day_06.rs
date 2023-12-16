use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Counter {
    elf: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "elf on a shelf"))]
    elf_shelf: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "shelf with no elf on it"))]
    shelf: Option<i32>,
}

/// # Task 1: Never count on an elf
/// Elves are notorious for their hide-and-seek skills, and now they've
/// hidden themselves in strings of text!
/// Create an endpoint /6 that takes a POST request with a raw string as
/// input and count how many times the substring "elf" appears.
/// The output should be a JSON object containing the count of the string "elf".
pub async fn elf_count(body: String) -> (StatusCode, Json<Counter>) {
    let count = body.as_str().matches("elf").count();

    (
        StatusCode::OK,
        Json(Counter {
            elf: count as i32,
            elf_shelf: None,
            shelf: None,
        }),
    )
}

/// # Task 2: Shelf under an elf? (200 bonus points)
/// Add two fields to the response that counts:
/// The number of occurrences of the string "elf on a shelf"
/// in a field with the same name.
/// The number of shelves that don't have an elf on it.
/// That is, the number of strings "shelf" that are not preceded
/// by the string "elf on a ". Put this count in the field "shelf with no elf on it".
pub async fn elf_shelf_count(body: String) -> (StatusCode, Json<Counter>) {
    let elf_count = body.as_str().matches("elf").count();
    let elf_on_a_shelf_count = body.as_str().matches("elf on a shelf").count();
    let on_a_shelf_count = body.as_str().matches("on a shelf").count();
    let elf_shelf_count = elf_on_a_shelf_count + on_a_shelf_count;
    let shelf_count = body.as_str().matches("shelf").count() - on_a_shelf_count;

    println!("e {elf_count} eoas {elf_on_a_shelf_count} oas {on_a_shelf_count} es {elf_shelf_count} s {shelf_count}");

    let (elf_shelf, shelf) = match (elf_shelf_count, shelf_count) {
        (0, _) => (None, None),
        (_, _) => (Some(on_a_shelf_count as i32), Some((shelf_count) as i32)),
    };

    (
        StatusCode::OK,
        Json(Counter {
            elf: elf_count as i32,
            elf_shelf,
            shelf,
        }),
    )
}
