use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AddProvider {
    #[validate(length(min = 1, max = 50, message = "请输入正确的名称"))]
    pub name: String,

    #[validate(range(min = 1, message = "请输入正确的续期时间"))]
    pub renew_days: i32,

    #[validate(range(min = 1, message = "请输入正确的通知时间"))]
    pub notify_days: i32,
}

#[derive(Deserialize, Validate)]
pub struct EditProvider {
    #[validate(length(min = 20, max = 20, message = "请输入正确的ID"))]
    pub id: String,

    #[validate(length(min = 1, max = 50, message = "请输入正确的名称"))]
    pub name: String,

    #[validate(range(min = 1, message = "请输入正确的续期时间"))]
    pub renew_days: i32,

    #[validate(range(min = 1, message = "请输入正确的通知时间"))]
    pub notify_days: i32,
}

#[derive(Deserialize)]
pub enum ListProviderSort {
    #[serde(rename = "name")]
    Name,

    #[serde(rename = "name_desc")]
    NameDesc,

    #[serde(rename = "id")]
    ID,

    #[serde(rename = "id_desc")]
    IDDesc,
}

#[derive(Deserialize)]
pub struct ListProvider {
    pub name: Option<String>,
    pub sort: Option<ListProviderSort>,
}
