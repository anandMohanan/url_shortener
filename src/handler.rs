use askama::Template;
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Json,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

use crate::{app_state::AppState, template::HomeTemplate, utils};

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

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    tracing::debug!("{}", rand_string);

    utils::set(
        rand_string.as_str(),
        payload.url.as_str(),
        appstate.redis.clone(),
    )
    .await;
    Ok(Json(rand_string))
}

pub async fn render_index() -> impl IntoResponse {
    let template = HomeTemplate {};
    let html = template.render().unwrap();
    (StatusCode::OK, Html(html).into_response())
}

pub async fn redirect(
    State(appstate): State<AppState>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    let result = utils::get(code.as_str(), appstate.redis.clone())
        .await
        .unwrap();
    Redirect::to(&result)
}
