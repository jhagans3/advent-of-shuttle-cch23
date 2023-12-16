use axum::http::StatusCode;
use axum_extra::extract::cookie::CookieJar;
use base64::{engine::general_purpose, Engine as _};
use std::str::from_utf8;

/// Task 1: Based encoding, 64th edition
/// Santa's secret cookie recipe is hidden in an encoded message,
/// and he's looking to you for help. He's sending a GET request to
/// your server with a Cookie header.
/// What you need to do is parse the Cookie header, decode the value
/// in the recipe field, and return it.
/// Make an endpoint /7/decode that extracts the Cookie HTTP header.
pub async fn base64_decoding(jar: CookieJar) -> (StatusCode, String) {
    //     println!("Cookie JAR: {jar:?}");
    let recipe_cookie_string = jar.get("recipe").unwrap().value();
    println!("Cookie string: {recipe_cookie_string}");

    let recipe_bytes = general_purpose::URL_SAFE
        .decode(recipe_cookie_string)
        .unwrap();

    let recipe_str = from_utf8(&recipe_bytes)
        .unwrap()
        .replace("\\", "")
        .to_string();

    println!("Decode: {recipe_str}");

    (StatusCode::OK, recipe_str)
}
