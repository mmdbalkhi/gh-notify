use reqwest::Client;
use serde_json::{from_str, Value};

pub async fn request(api_key: &str) -> Value {
    let client = Client::new();
    let body = client
        .get("https://api.github.com/notifications?")
        .header("User-Agent", "gh-notify.rs")
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    from_str(&body).unwrap()
}
