use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json, debug_handler,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{app_state::AppState, utils};
use redis::Commands;

#[derive(Debug, Deserialize)]
pub struct CreateUrl {
    pub url: String,
}


#[debug_handler]
pub async fn post_url_to_redis(
    State(appstate): State<AppState>,
    Json(payload): Json<CreateUrl>,
) -> Result<Json<String>, (StatusCode, String)> {
    println!("{:#?}", payload);

    utils::set("hello", "2", appstate.redis.clone()).await;
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    println!("{}", rand_string);
    Ok(Json(rand_string))
}

pub async fn get_url(State(appstate): State<AppState>) -> impl IntoResponse {
    let result = utils::get("hello", appstate.redis.clone()).await;
    println!("{:#?}", result.unwrap());
    Html("hi")
}
// pub async fn get_url(Path(name): Path<String>) -> impl IntoResponse {
//     Html("hi")
// }
