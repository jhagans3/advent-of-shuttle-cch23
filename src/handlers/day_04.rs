use axum::{extract, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Reindeer {
    name: String,
    strength: i32,
}

/// # Task 1: Reindeer cheer
/// The task is to create a POST endpoint /4/strength that calculates the combined
/// strength of a group of reindeer, so that Santa knows if they can pull his sled
/// through the skies.
///
/// The input to the endpoint is a JSON array containing information about
/// each reindeer. Each reindeer is represented as an object with two
/// attributes: "name" (string) and "strength" (integer). Collect the strength of
/// each reindeer and respond with the sum.
pub async fn reindeer_cheer(
    extract::Json(reindeers): extract::Json<Vec<Reindeer>>,
) -> (StatusCode, String) {
    let total_strength = reindeers.iter().fold(0, |acc, r| acc + r.strength);

    (StatusCode::OK, format!("{total_strength}"))
}
