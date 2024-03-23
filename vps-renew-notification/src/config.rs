use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub dsn: String,
    pub max_conns: u32,
}

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}
#[derive(Deserialize)]
pub struct JwtConfig {
    pub secret_key: String,
    pub expire: u32,
}
#[derive(Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub chat_id: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub db: DatabaseConfig,
    pub web: WebConfig,
    pub jwt: JwtConfig,
    pub bot: BotConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
