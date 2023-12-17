use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    Router,
};
use redis::Connection;

use crate::{
    app_state::AppState,
    handler::{get_url, post_url_to_redis},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/createUrl", post(post_url_to_redis))
        .route("/getUrl", get(get_url))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}
