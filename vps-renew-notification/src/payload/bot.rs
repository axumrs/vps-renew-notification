use serde::Deserialize;

#[derive(Deserialize)]
pub struct BotMessage {
    pub text: String,
}
