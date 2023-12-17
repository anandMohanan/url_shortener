mod app_state;
mod handler;
mod route;
mod template;
mod utils;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::route::create_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_templates=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let redis = utils::connect().await;
    let app_state_redis = app_state::AppState { redis };
    let app = create_router(app_state_redis);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
