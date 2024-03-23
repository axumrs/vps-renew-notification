use axum::response::IntoResponse;

pub async fn list() -> impl IntoResponse {
    "provider list"
}
