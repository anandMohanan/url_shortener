use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    handler::{post_url_to_redis, redirect, render_index},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/createUrl", post(post_url_to_redis))
        .route("/", get(render_index))
        .route("/:code", get(redirect))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}
