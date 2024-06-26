use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{jwt, Error};

#[derive(Serialize, Deserialize)]
pub struct JsonResp<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T: Serialize + DeserializeOwned> JsonResp<T> {
    pub fn new(code: i32, msg: String, data: T) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(0, "OK".to_string(), data)
    }
}

impl JsonResp<()> {
    pub fn ok_empty() -> Self {
        Self::ok(())
    }

    pub fn failed(code: i32, msg: String) -> Self {
        Self::new(code, msg, ())
    }

    pub fn err(err: Error) -> Self {
        Self::failed(err.code(), err.message)
    }
}

#[derive(Deserialize, Serialize)]
pub struct IDResp {
    pub id: String,
}
#[derive(Deserialize, Serialize)]
pub struct AffResp {
    pub aff: u64,
}
#[derive(Deserialize, Serialize)]
pub struct BotResp {
    pub code: u16,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResp {
    pub auth: jwt::AuthBody,
    pub data: jwt::UserClaimsData,
}
