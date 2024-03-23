use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub enum ListVpsSort {
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
pub struct ListVps {
    pub name: Option<String>,
    pub provider_id: Option<String>,
    pub sort: Option<ListVpsSort>,
}

#[derive(Deserialize, Validate)]
pub struct AddVps {
    #[validate(length(min = 20, max = 20, message = "请输入正确的ID"))]
    pub provider_id: String,

    #[validate(length(min = 1, max = 50, message = "请输入正确的名称"))]
    pub name: String,

    pub expire: String,
}
#[derive(Deserialize, Validate)]
pub struct EditVps {
    #[validate(length(min = 20, max = 20, message = "请输入正确的ID"))]
    pub id: String,

    #[validate(length(min = 20, max = 20, message = "请输入正确的ID"))]
    pub provider_id: String,

    #[validate(length(min = 1, max = 50, message = "请输入正确的名称"))]
    pub name: String,

    pub expire: String,
}
