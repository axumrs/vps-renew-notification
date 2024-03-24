use reqwest::StatusCode;

use crate::{Error, Result};

pub async fn send_message(token: &str, chat_id: &str, text: &str) -> Result<StatusCode> {
    let api_url = format!("https://api.telegram.org/bot{token}/sendMessage");
    let cli = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(Error::from)?;

    let body = [("chat_id", chat_id), ("text", text)];

    let resp = cli
        .post(&api_url)
        .form(&body)
        .send()
        .await
        .map_err(Error::from)?;

    // tracing::debug!("{:?}", resp);

    Ok(resp.status())
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test_bot_send_message() {
        let token = std::env::var("BOT_TOKEN").unwrap();
        let chat_id = std::env::var("BOT_CHAT_ID").unwrap();
        let code = super::send_message(&token, &chat_id, "Hello, 世界")
            .await
            .unwrap();
        println!("{code}");
    }
}
