use axum::{extract::Path, http::StatusCode};
use serde_json::Value;

/// # Task 1: IT'S PIKACHU!
/// Your quest is to add a GET endpoint /8/weight/<pokedex_number> that,
/// given a pokédex number, responds with the corresponding Pokémon's
/// weight in kilograms as a number in plain text.
pub async fn weight(Path(pokedex_number): Path<u32>) -> (StatusCode, String) {
    let pokedex_uri = format!("https://pokeapi.co/api/v2/pokemon/{pokedex_number}");
    let pokedex_response = reqwest::get(pokedex_uri)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let json: Value = serde_json::from_str(&pokedex_response).unwrap();

    let pokemon_weight = match json.get("weight") {
        Some(weight) => weight.to_string(),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "weight not found in json".to_string(),
            )
        }
    };

    let pokemon_weight_string = match pokemon_weight.parse::<f64>() {
        Ok(weight) => {
            let wf = weight / 10.0;
            format!("{wf}")
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    (StatusCode::OK, pokemon_weight_string)
}

/// # Task 2: That's gonna leave a dent (160 bonus points)
/// Once the Pokémon's weight is retrieved, Santa needs you to calculate
/// the momentum it would have at the time of impact with the floor if
/// dropped from a 10-meter high chimney (so that he knows if he needs
/// to climb down or if he can just drop it).
///
/// Keep in mind that the gravity of Earth that Santa has in his physics
/// book was measured close to the North Pole. This could explain why his
/// calculations are a bit off sometimes, but he still wants you to use it.
///
/// The momentum, measured in Newton-seconds, signifies the force the Pokémon
/// would exert upon meeting the floor beneath the 10-meter high chimney.
/// The GET endpoint /8/drop/<pokedex_number> shall respond with a plain text
/// floating point number.
///
/// Use gravitational acceleration g = 9.825 m/s². Ignore air resistance.
pub async fn drop(Path(pokedex_number): Path<u32>) -> (StatusCode, String) {
    let pokedex_uri = format!("https://pokeapi.co/api/v2/pokemon/{pokedex_number}");
    let pokedex_response = reqwest::get(pokedex_uri)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let json: Value = serde_json::from_str(&pokedex_response).unwrap();

    let pokemon_weight = match json.get("weight") {
        Some(weight) => weight.to_string(),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "weight not found in json".to_string(),
            )
        }
    };

    let pokemon_weight_gravity = match pokemon_weight.parse::<f64>() {
        Ok(weight) => {
            let wf = weight / 10.0;
            let acceleration = (2.0f64 * 9.825 * 10.0).sqrt();
            let gravity = acceleration * wf;
            format!("{gravity}")
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    (StatusCode::OK, pokemon_weight_gravity)
}
