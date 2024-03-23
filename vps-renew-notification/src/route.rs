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
        .route(
            "/",
            get(handler::provider::list)
                .post(handler::provider::add)
                .put(handler::provider::edit),
        )
        .route(
            "/:id",
            get(handler::provider::find).delete(handler::provider::del),
        )
        .with_state(state)
}

pub fn vps(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/",
            get(handler::vps::list)
                .post(handler::vps::add)
                .put(handler::vps::edit),
        )
        .route("/:id", get(handler::vps::find).delete(handler::vps::del))
        .with_state(state)
}

pub fn manage(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/provider", provider(state.clone()))
        .nest("/vps", vps(state.clone()))
        .layer(middleware::from_fn_with_state(state, mw::get_user_auth))
}
