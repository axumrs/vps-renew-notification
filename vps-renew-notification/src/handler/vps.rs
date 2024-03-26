use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
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
) -> Result<Json<JsonResp<Vec<model::VPSWithProvider>>>> {
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

pub async fn del(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<JsonResp<AffResp>>> {
    let handler_name = "vps/del";
    let pool = get_conn(&state);
    let aff = db::vps::delete(&*pool, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(Json(JsonResp::ok(AffResp { aff })))
}

pub async fn renew(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<JsonResp<AffResp>>> {
    let handler_name = "vps/del";
    let pool = get_conn(&state);
    let mut tx = pool
        .begin()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let v = db::vps::find(&mut *tx, &id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    if v.is_none() {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(Error::not_exists("不存在的VPS"));
    }
    let v = v.unwrap();

    // 查找续期时间
    let provider = db::provider::find(&mut *tx, &v.provider_id)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    if provider.is_none() {
        tx.rollback()
            .await
            .map_err(Error::from)
            .map_err(log_error(handler_name))?;
        return Err(Error::not_exists("不存在的服务商"));
    }
    let provider = provider.unwrap();

    // 修改时间
    let duration = chrono::TimeDelta::try_days(provider.renew_days as i64).unwrap();
    //let expire = v.expire + duration;
    let expire = (chrono::Local::now() + duration)
        .format("%Y-%m-%dT00:00:00+08:00")
        .to_string();
    let expire = chrono_from_str(&expire).map_err(log_error(handler_name))?;
    tracing::debug!(
        "duration: {:?}, expire: {:?}, renew_days: {}",
        duration,
        expire,
        provider.renew_days
    );

    let v = model::VPS { expire, ..v };
    let aff = match db::vps::update(&mut *tx, &v).await {
        Ok(aff) => aff,
        Err(err) => {
            tx.rollback()
                .await
                .map_err(Error::from)
                .map_err(log_error(handler_name))?;
            tracing::error!("{handler_name} - {:?}", err);
            return Err(Error::from(err));
        }
    };

    tx.commit()
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;
    Ok(Json(JsonResp::ok(AffResp { aff })))
}
