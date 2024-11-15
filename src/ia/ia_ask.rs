use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct IaResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
    pub context: Vec<i32>
}

pub async fn ask(prompt: String, model: String, context: Option<Vec<i32>>) -> Result<IaResponse, Box<dyn std::error::Error>> {
    let client = Client::new();

    let payload = json!({
        "model": model,
        "prompt": prompt,
        "stream": false,
        "context": context,
    });

    let response = client
        .post(format!("{}/generate", &crate::common::LLAMA_API_URL))
        .json(&payload)
        .send()
        .await
        .unwrap();

    let json: IaResponse = response.json().await.unwrap();
    Ok(json)
}