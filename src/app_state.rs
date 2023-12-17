use crate::utils::RedisPool;
use axum::extract::FromRef;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub redis: RedisPool,
}
