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
