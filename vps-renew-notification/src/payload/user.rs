use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserChangePassword {
    #[validate(length(min = 20, max = 20, message = "请输入正确的ID"))]
    pub id: String,
    pub password: String,
    pub new_password: String,
    pub re_password: String,
}
