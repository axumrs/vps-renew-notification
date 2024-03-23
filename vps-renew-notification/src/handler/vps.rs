use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use validator::Validate;

use crate::{chrono_from_str, db, model, payload, AppState, Error, IDResp, JsonResp, Result};

use super::helper::{get_conn, log_error};

pub async fn list() -> impl IntoResponse {
    "vps list"
}

pub async fn add(
    State(state): State<Arc<AppState>>,
    Json(p): Json<payload::AddVps>,
) -> Result<Json<JsonResp<IDResp>>> {
    let handler_name = "vps/add";

    p.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let pool = get_conn(&state);

    let expire = chrono_from_str(&p.expire).map_err(log_error(handler_name))?;

    let m = model::VPS {
        provider_id: p.provider_id,
        name: p.name,
        expire,
        dateline: chrono::Local::now(),
        ..Default::default()
    };
    let id = db::vps::create(&*pool, &m)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(Json(JsonResp::ok(IDResp { id })))
}
