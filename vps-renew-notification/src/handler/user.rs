use std::sync::Arc;

use axum::{extract::State, Json};
use validator::Validate;

use crate::{
    db, filter,
    handler::helper::{get_conn, log_error},
    model, password, payload, AffResp, AppState, Error, JsonResp, Result,
};

pub async fn change_password(
    State(state): State<Arc<AppState>>,
    Json(p): Json<payload::UserChangePassword>,
) -> Result<Json<JsonResp<AffResp>>> {
    let handler_name = "user/change_password";

    p.validate()
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if p.password == p.new_password {
        return Err(Error::invalid_parameter("玩呢？一样的密码改个鬼呀！"));
    }

    if p.new_password != p.re_password {
        return Err(Error::invalid_parameter("两次输入的密码不一致"));
    }

    let pool = get_conn(&state);

    let u = db::user::find(&*pool, &filter::UserFindBy::ID(&p.id))
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    if u.is_none() {
        return Err(Error::not_exists("不存在的用户"));
    }
    let u = u.unwrap();

    if !password::verify(&p.password, &u.password).map_err(log_error(handler_name))? {
        return Err(Error::incorrect_auth());
    }

    let pwd = password::hash(&p.new_password).map_err(log_error(handler_name))?;

    let u = model::User { password: pwd, ..u };

    let aff = db::user::update(&*pool, &u)
        .await
        .map_err(Error::from)
        .map_err(log_error(handler_name))?;

    Ok(Json(JsonResp::ok(AffResp { aff })))
}
