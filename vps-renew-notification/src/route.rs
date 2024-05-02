use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post, put},
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
        .route(
            "/:id",
            get(handler::vps::find)
                .delete(handler::vps::del)
                .patch(handler::vps::renew),
        )
        .route("/batch-renew", post(handler::vps::batch_renew))
        .with_state(state)
}
pub fn user(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/change-password", put(handler::user::change_password))
        .with_state(state)
}
pub fn bot(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/send-message", post(handler::bot::send_message))
        .with_state(state)
}

pub fn manage(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/provider", provider(state.clone()))
        .nest("/vps", vps(state.clone()))
        .nest("/user", user(state.clone()))
        .nest("/bot", bot(state.clone()))
        .layer(middleware::from_fn_with_state(state, mw::get_user_auth))
}
