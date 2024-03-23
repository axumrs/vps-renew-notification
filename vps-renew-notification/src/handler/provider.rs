use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use validator::Validate;

use crate::{db, filter, model, payload, AffResp, AppState, Error, IDResp, JsonResp, Result};

use super::helper::{get_conn, log_error};

pub async fn list(
    State(state): State<Arc<AppState>>,
    Query(p): Query<payload::ListProvider>,
) -> Result<Json<JsonResp<Vec<model::Provider>>>> {
    let handler_name = "provider/list";
    let pool = get_conn(&state);
    let sort: Option<String> = if let Some(s) = p.sort {
        Some(
            match s {
                payload::ListProviderSort::ID => "id",
                payload::ListProviderSort::IDDesc => "id DESC",
                payload::ListProviderSort::Name => "name",
                payload::ListProviderSort::NameDesc => "name DESC",
            }
            .to_string(),
        )
    } else {
        None
    };
    let f = filter::ProviderListFilter { name: p.name, sort };
    let data = db::provider::list(&*pool, &f)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(Json(JsonResp::ok(data)))
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

pub async fn find(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<JsonResp<model::Provider>>> {
    let handler_name = "provider/del";

    let pool = get_conn(&state);

    let p = db::provider::find(&*pool, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if p.is_none() {
        return Err(Error::not_exists("不存在的服务商"));
    }

    Ok(Json(JsonResp::ok(p.unwrap())))
}

pub async fn del(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<JsonResp<AffResp>>> {
    let handler_name = "provider/del";

    let pool = get_conn(&state);

    let aff = db::provider::delete(&*pool, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(Json(JsonResp::ok(AffResp { aff })))
}
