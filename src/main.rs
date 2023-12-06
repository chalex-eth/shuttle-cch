use axum::{
    extract::Path,
    //http::StatusCode,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/1/*path", get(result_1))
        .route("/4/strength", post(result_4_strength))
        .route("/4/contest", post(result_4_contest));

    Ok(router.into())
}

//1

async fn result_1(Path(input): Path<String>) -> Response {
    let nums: Vec<i32> = input
        .split('/')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    let result = nums.iter().fold(0, |acc, &num| acc ^ num).pow(3);
    println!("result: {}", result);
    result.to_string().into_response()
}

//4.1

#[derive(Deserialize)]
pub struct StrengthInput {
    pub name: String,
    pub strength: i32,
}

async fn result_4_strength(Json(input): Json<Vec<StrengthInput>>) -> Response {
    let result = input.iter().map(|s| s.strength).sum::<i32>();
    result.to_string().into_response()
}

//4.2

#[derive(Deserialize)]
pub struct ContestInput {
    pub name: String,
    pub strength: i32,
    pub speed: f32,
    pub height: i32,
    pub antler_width: i32,
    pub snow_magic_power: i32,
    pub favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies_eaten_yesterday: i32,
}

#[derive(Serialize)]
pub struct ContestOutput {
    pub fastest: String,
    pub tallest: String,
    pub magician: String,
    pub consumer: String,
}

struct ContestWinners {
    fastest: (f32, String),
    tallest: (i32, String),
    magician: (i32, String),
    consumer: (i32, String, String),
}

async fn result_4_contest(Json(input): Json<Vec<ContestInput>>) -> Response {
    let winners = input.iter().fold(
        ContestWinners {
            fastest: (0.0, String::new()),
            tallest: (0, String::new()),
            magician: (0, String::new()),
            consumer: (0, String::new(), String::new()),
        },
        |mut acc, reindeer| {
            if reindeer.speed > acc.fastest.0 {
                acc.fastest = (reindeer.speed, reindeer.name.clone());
            }
            if reindeer.height > acc.tallest.0 {
                acc.tallest = (reindeer.height, reindeer.name.clone());
            }
            if reindeer.snow_magic_power > acc.magician.0 {
                acc.magician = (reindeer.snow_magic_power, reindeer.name.clone());
            }
            if reindeer.candies_eaten_yesterday > acc.consumer.0 {
                acc.consumer = (
                    reindeer.candies_eaten_yesterday,
                    reindeer.name.clone(),
                    reindeer.favorite_food.clone(),
                );
            }
            acc
        },
    );

    let result = ContestOutput {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            winners.fastest.0, winners.fastest.1
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            winners.tallest.1, winners.tallest.0
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            winners.magician.1, winners.magician.0
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            winners.consumer.1, winners.consumer.2
        ),
    };

    Json(result).into_response()
}
