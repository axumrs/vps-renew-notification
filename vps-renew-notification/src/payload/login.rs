use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Login {
    #[validate(length(max = 50, message = "请输入用户名"))]
    pub username: String,
    pub password: String,
}
