use serde::Serialize;
use reqwest::Client;

#[derive(Debug, Serialize)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

#[derive(Debug, Serialize)]
pub struct Embed {
    pub title: String,
    pub url: String,
    pub fields: Vec<EmbedField>,
    pub thumbnail: EmbedThumbnail,
}

#[derive(Debug, Serialize)]
pub struct EmbedThumbnail {
    pub(crate) url: String,
}

#[derive(Debug, Serialize)]
pub struct WebhookMessage {
    pub content: String,
    pub embeds: Vec<Embed>,
}

pub async fn send_embed(webhook_url: &str, message: WebhookMessage) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let res = client.post(webhook_url).json(&message).send().await.unwrap();

    if res.status().is_success() {
        println!("Message sent successfully");
    } else {
        let status = res.status();
        let body = res.text().await?;
        println!("{:?}", message);
        println!("Failed to send message: Status: {:?}, Body: {:?}", status, body);
    }

    Ok(())
}
