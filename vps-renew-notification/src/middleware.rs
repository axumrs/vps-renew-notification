use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, HeaderMap},
    middleware::Next,
    response::Response,
};

use crate::{jwt, AppState, Error, JsonResp};

async fn token_from_header(h: &HeaderMap) -> Option<String> {
    let auth_header = h
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());
    if auth_header.is_none() {
        return None;
    }
    let auth_header_arr: Vec<&str> = auth_header.unwrap_or_default().split(" ").collect();
    // tracing::debug!("{:?}", auth_header_arr);

    if auth_header_arr.len() != 2 {
        return None;
    }

    Some(auth_header_arr[1].to_string())
}

async fn claims_from_header(
    h: &HeaderMap,
    state: &AppState,
) -> Option<jwt::Claims<jwt::UserClaimsData>> {
    let token = token_from_header(h).await;
    if token.is_none() {
        return None;
    }
    let token = token.unwrap();

    match jwt::decode(&token, &state.cfg.jwt.secret_key) {
        Ok(cd) => Some(cd),
        Err(_) => None,
    }
}

pub struct UserAuth(pub jwt::UserClaimsData);

pub async fn get_user_auth(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    let c = claims_from_header(&req.headers(), &state).await;

    let resp = match c {
        Some(_) => next.run(req).await,
        None => Response::new(Body::from(
            serde_json::to_string(&JsonResp::err(Error::unauthorized())).unwrap(),
        )),
    };
    resp
}
