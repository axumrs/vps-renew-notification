use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use validator::Validate;

use crate::{
    chrono_from_str, db, filter, model, payload, AffResp, AppState, Error, IDResp, JsonResp, Result,
};

use super::helper::{get_conn, log_error};

pub async fn list(
    State(state): State<Arc<AppState>>,
    Query(p): Query<payload::ListVps>,
) -> Result<Json<JsonResp<Vec<model::VPS>>>> {
    let handler_name = "vps/list";
    let pool = get_conn(&state);
    let sort: Option<String> = if let Some(s) = p.sort {
        Some(
            match s {
                payload::ListVpsSort::ID => "id",
                payload::ListVpsSort::IDDesc => "id DESC",
                payload::ListVpsSort::Name => "name",
                payload::ListVpsSort::NameDesc => "name DESC",
                payload::ListVpsSort::Expire => "expire",
                payload::ListVpsSort::ExpireDesc => "expire DESC",
            }
            .to_string(),
        )
    } else {
        None
    };
    let f = filter::VpsListFilter {
        name: p.name,
        provider_id: p.provider_id,
        sort,
    };
    let data = db::vps::list(&*pool, &f)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(Json(JsonResp::ok(data)))
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

pub async fn edit(
    State(state): State<Arc<AppState>>,
    Json(p): Json<payload::EditVps>,
) -> Result<Json<JsonResp<AffResp>>> {
    let handler_name = "vps/edit";

    p.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let pool = get_conn(&state);

    let expire = chrono_from_str(&p.expire).map_err(log_error(handler_name))?;

    let m = model::VPS {
        id: p.id,
        provider_id: p.provider_id,
        name: p.name,
        expire,
        ..Default::default()
    };
    let aff = db::vps::update(&*pool, &m)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(Json(JsonResp::ok(AffResp { aff })))
}

pub async fn find(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<JsonResp<model::VPS>>> {
    let handler_name = "vps/find";
    let pool = get_conn(&state);
    let v = db::vps::find(&*pool, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    if v.is_none() {
        return Err(Error::not_exists("不存在的VPS"));
    }
    Ok(Json(JsonResp::ok(v.unwrap())))
}
