use std::sync::Arc;

pub struct AppState {
    pub pool: Arc<sqlx::PgPool>,
    pub cfg: Arc<crate::config::Config>,
}
