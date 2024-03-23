use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{handler, middleware as mw, AppState};

pub fn auth(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(handler::auth::login))
        .with_state(state)
}

pub fn provider(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handler::provider::list))
        .with_state(state)
}

pub fn vps(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handler::vps::list))
        .with_state(state)
}

pub fn manage(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/provider", provider(state.clone()))
        .nest("/vps", vps(state.clone()))
        .layer(middleware::from_fn_with_state(state, mw::get_user_auth))
}
