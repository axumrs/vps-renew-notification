use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use validator::Validate;

use crate::{db, model, payload, AffResp, AppState, Error, IDResp, JsonResp, Result};

use super::helper::{get_conn, log_error};

pub async fn list() -> impl IntoResponse {
    "provider list"
}

pub async fn add(
    State(state): State<Arc<AppState>>,
    Json(p): Json<payload::AddProvider>,
) -> Result<Json<JsonResp<IDResp>>> {
    let handler_name = "provider/add";

    p.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let pool = get_conn(&state);

    let m = model::Provider {
        name: p.name,
        renew_days: p.renew_days,
        notify_days: p.notify_days,
        dateline: chrono::Local::now(),
        ..Default::default()
    };

    let id = db::provider::create(&*pool, &m)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(Json(JsonResp::ok(IDResp { id })))
}

pub async fn edit(
    State(state): State<Arc<AppState>>,
    Json(p): Json<payload::EditProvider>,
) -> Result<Json<JsonResp<AffResp>>> {
    let handler_name = "provider/edit";

    p.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let pool = get_conn(&state);

    let m = model::Provider {
        id: p.id,
        name: p.name,
        renew_days: p.renew_days,
        notify_days: p.notify_days,
        ..Default::default()
    };

    let aff = db::provider::update(&*pool, &m)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(Json(JsonResp::ok(AffResp { aff })))
}
