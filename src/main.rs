use axum::{
    extract::Path,
    //http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/1/*path", get(result));

    Ok(router.into())
}

async fn result(Path(input): Path<String>) -> Response {
    let nums: Vec<i32> = input
        .split('/')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    let result = nums.iter().fold(0, |acc, &num| acc ^ num).pow(3);
    println!("result: {}", result);
    result.to_string().into_response()
}
