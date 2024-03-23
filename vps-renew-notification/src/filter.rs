pub struct ProviderListFilter {
    pub name: Option<String>,
    pub sort: Option<String>,
}
pub struct VpsListFilter {
    pub name: Option<String>,
    pub provider_id: Option<String>,
    pub sort: Option<String>,
}

pub enum UserFindBy<'a> {
    ID(&'a str),
    Username(&'a str),
}
