use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub renew_days: i32,
    pub notify_days: i32,
    pub dateline: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct VPS {
    pub id: String,
    pub provider_id: String,
    pub name: String,
    pub expire: chrono::DateTime<chrono::Local>,
    pub dateline: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub dateline: chrono::DateTime<chrono::Local>,
}
