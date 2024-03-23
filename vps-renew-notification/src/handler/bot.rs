use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{payload, tg, AppState, BotResp, JsonResp, Result};

use super::helper::log_error;

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(p): Json<payload::BotMessage>,
) -> Result<Json<JsonResp<BotResp>>> {
    let handler_name = "bot/send_message";
    let status = tg::send_message(&state.cfg.bot.token, &state.cfg.bot.chat_id, &p.text)
        .await
        .map_err(log_error(handler_name))?;
    Ok(Json(JsonResp::ok(BotResp {
        code: status.as_u16(),
    })))
}
