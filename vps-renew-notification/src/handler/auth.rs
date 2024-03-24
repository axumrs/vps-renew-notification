use std::sync::Arc;

use axum::{extract::State, Json};
use validator::Validate;

use crate::{db, filter, jwt, password, payload, AppState, Error, JsonResp, LoginResp, Result};

use super::helper::{get_conn, log_error};

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(p): Json<payload::Login>,
) -> Result<Json<JsonResp<LoginResp>>> {
    let handler_name = "auth/login";

    p.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    let pool = get_conn(&state);
    let u = db::user::find(&*pool, &filter::UserFindBy::Username(&p.username))
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if u.is_none() {
        return Err(Error::incorrect_auth());
    }
    let u = u.unwrap();

    if !password::verify(&p.password, &u.password).map_err(log_error(handler_name))? {
        return Err(Error::incorrect_auth());
    }

    let ucd = jwt::UserClaimsData {
        id: u.id,
        username: u.username,
    };
    let auth = jwt::encode(&state.cfg.jwt.secret_key, state.cfg.jwt.expire, ucd.clone())?;
    let data = LoginResp { auth, data: ucd };

    Ok(Json(JsonResp::ok(data)))
}
