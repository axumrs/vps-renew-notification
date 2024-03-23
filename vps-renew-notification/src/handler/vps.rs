use axum::response::IntoResponse;

pub async fn list() -> impl IntoResponse {
    "vps list"
}
