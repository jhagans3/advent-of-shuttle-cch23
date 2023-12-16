use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, PartialEq, PartialOrd)]
pub struct Reindeer {
    name: String,
    strength: i32,
    speed: Option<f32>,
    height: Option<i32>,
    antler_width: Option<i32>,
    snow_magic_power: Option<i32>,
    favorite_food: Option<String>,
    #[serde(rename(deserialize = "cAnD13s_3ATeN-yesT3rdAy"))]
    candies_eaten_yesterday: Option<i32>,
}

#[derive(Serialize)]
pub struct Winner {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
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

/// # Task 2: Cursed candy eating contest (150 bonus points)
/// This time, there is some more data for each reindeer (see example).
/// The endpoint is called /4/contest, because the reindeer are now going
/// to compare the attributes of the reindeer and present a summary of the winners.
/// There is at least one reindeer participating in the contest, and there is
/// never a tie for first place.
///
/// For some reason, one of the field names in the input seems to still be a
/// bit corrupted from the incident. Guess we just have to deal with it anyways.
pub async fn candy_contest(
    extract::Json(reindeers): extract::Json<Vec<Reindeer>>,
) -> (StatusCode, Json<Winner>) {
    let fastest_reindeer = reindeers
        .iter()
        .max_by(|r1, r2| r1.speed.partial_cmp(&r2.speed).unwrap())
        .unwrap();

    let fastest = format!(
        "Speeding past the finish line with a strength of {} is {}",
        fastest_reindeer.strength, fastest_reindeer.name
    );

    let tallest_reindeer = reindeers
        .iter()
        .max_by(|r1, r2| r1.height.partial_cmp(&r2.height).unwrap())
        .unwrap();

    let tallest = format!(
        "{} is standing tall with his {:?} cm wide antlers",
        tallest_reindeer.name,
        tallest_reindeer.antler_width.unwrap()
    );

    let magician_reindeer = reindeers
        .iter()
        .max_by(|r1, r2| {
            r1.snow_magic_power
                .partial_cmp(&r2.snow_magic_power)
                .unwrap()
        })
        .unwrap();
    let magician = format!(
        "{} could blast you away with a snow magic power of {:?}",
        magician_reindeer.name,
        magician_reindeer.snow_magic_power.unwrap()
    );

    let consumer_reindeer = reindeers
        .iter()
        .max_by(|r1, r2| {
            r1.candies_eaten_yesterday
                .partial_cmp(&r2.candies_eaten_yesterday)
                .unwrap()
        })
        .unwrap();
    let consumer = format!(
        "{} ate lots of candies, but also some {}",
        consumer_reindeer.name,
        consumer_reindeer.favorite_food.clone().unwrap()
    );

    let winner = Winner {
        fastest,
        tallest,
        magician,
        consumer,
    };

    (StatusCode::OK, Json(winner))
}
