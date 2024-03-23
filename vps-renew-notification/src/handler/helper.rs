use std::sync::Arc;

use crate::{AppState, Error};

pub fn get_conn(state: &Arc<AppState>) -> Arc<sqlx::PgPool> {
    state.pool.clone()
}

pub fn log_error(handler_name: &str) -> Box<dyn Fn(Error) -> Error> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("👉 [{}] - {:?}", handler_name, err);
        err
    })
}
