use axum::http::StatusCode;
use axum_extra::extract::cookie::CookieJar;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::from_utf8};

/// # Task 1: Based encoding, 64th edition
/// Santa's secret cookie recipe is hidden in an encoded message,
/// and he's looking to you for help. He's sending a GET request to
/// your server with a Cookie header.
/// What you need to do is parse the Cookie header, decode the value
/// in the recipe field, and return it.
/// Make an endpoint /7/decode that extracts the Cookie HTTP header.
pub async fn base64_decoding(jar: CookieJar) -> (StatusCode, String) {
    let recipe_cookie_string = match jar.get("recipe") {
        Some(s) => s.value(),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "recipe not found in cookie".to_string(),
            )
        }
    };
    println!("Cookie string: {recipe_cookie_string}");

    let recipe_bytes = match general_purpose::URL_SAFE.decode(recipe_cookie_string) {
        Ok(bytes) => bytes,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    let recipe_string = match from_utf8(&recipe_bytes) {
        Ok(s) => s.replace("\\", ""),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    println!("Decode: {recipe_string}");

    (StatusCode::OK, recipe_string)
}

#[derive(Clone, Debug, Deserialize)]
pub struct RecipePantry {
    recipe: HashMap<String, i64>,
    pantry: HashMap<String, i64>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Out {
    cookies: i64,
    pantry: HashMap<String, i64>,
}

/// #Task 2: The secret cookie recipe (120 bonus points)
/// Now that the recipe is decoded, Santa can get back to baking cookies!
/// Santa is not sure, however, if he has enough of each ingredient to
/// bake a cookie for every elf.
///
/// The same type of request as in Task 1 will be sent to a new endpoint,
/// /7/bake, but this time Santa needs your help to calculate the amount
/// of cookies he can bake with the ingredients he has in the pantry.
/// After decoding, parse the bytes as a JSON object (shape and keys can
/// be seen in the example below) and use that to calculate how many cookies
/// can be baked with the provided recipe and ingredients. Additionally,
/// return the amount of ingredients that would remain in the pantry after
/// the cookies have been baked.
pub async fn bake(jar: CookieJar) -> (StatusCode, String) {
    let cookie_str = match jar.get("recipe") {
        Some(s) => s.value(),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "recipe not found in cookie".to_string(),
            )
        }
    };

    let bytes = match general_purpose::URL_SAFE.decode(cookie_str) {
        Ok(bytes) => bytes,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    let recipe_pantry_json = match from_utf8(&bytes) {
        Ok(s) => s.replace("\\", ""),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    println!("Input json Object: {recipe_pantry_json}");

    let (recipe, pantry) = match serde_json::from_str(&recipe_pantry_json) {
        Ok(json_bytes) => {
            let rp: RecipePantry = json_bytes;
            (rp.recipe, rp.pantry)
        }
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    let number_of_servings = pantry
        .iter()
        .map(|(item, amount)| {
            let ingredient = recipe.get(item.as_str()).unwrap_or(&1);
            let servings = amount / ingredient;

            servings
        })
        .min()
        .unwrap_or(0);

    let unused_ingredients: HashMap<String, i64> = pantry
        .into_iter()
        .map(|(item, amount)| {
            let recipe_need = recipe.get(item.as_str()).unwrap_or(&0);
            let used_amount = recipe_need * number_of_servings;
            let new_amount = amount - used_amount;
	    println!("need {recipe_need} used {used_amount} #{number_of_servings} old {amount} new {new_amount}");

            (item, new_amount)
        })
        .collect();

    //     println!("Unused {unused_ingredients:?}");

    let res = match serde_json::to_string(&Out {
        cookies: number_of_servings,
        pantry: unused_ingredients,
    }) {
        Ok(v) => v.replace("\\", ""),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };

    println!("JSON RES: {res:?}");

    (StatusCode::OK, res)
}
