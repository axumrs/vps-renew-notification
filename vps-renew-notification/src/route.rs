use std::sync::Arc;

use axum::{middleware, routing::get, Router};

use crate::{handler, middleware as mw, AppState};

pub fn auth() -> Router {
    Router::new().route("/login", get(handler::auth::login))
}

pub fn provider() -> Router {
    Router::new().route("/", get(handler::provider::list))
}

pub fn vps() -> Router {
    Router::new().route("/", get(handler::vps::list))
}

pub fn manage(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/provider", provider())
        .nest("/vps", vps())
        .layer(middleware::from_fn_with_state(state, mw::get_user_auth))
}
