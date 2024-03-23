use std::sync::Arc;

use axum::Router;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use vps_renew_notification::{config, AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = config::Config::from_env().unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(cfg.db.max_conns)
        .connect(&cfg.db.dsn)
        .await
        .unwrap();

    let addr = cfg.web.addr.clone();

    let state = Arc::new(AppState {
        pool: Arc::new(pool),
        cfg: Arc::new(cfg),
    });

    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    let tcp_listener = TcpListener::bind(&addr).await.unwrap();

    tracing::info!("监听于 {}", addr);

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();
}
