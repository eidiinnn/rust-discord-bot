use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct IaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

pub async fn ask(prompt: String, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let payload = json!({
        "model": model,
        "prompt": prompt,
        "stream": false
    });

    let response = client
        .post(format!("{}/generate", &crate::common::LLAMA_API_URL))
        .json(&payload)
        .send()
        .await
        .unwrap();

    let json: IaResponse = response.json().await.unwrap();
    Ok(json.response)
}