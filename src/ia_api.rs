use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct IaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

pub async fn ia_ask(prompt: String, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let payload = json!({
        "model": model,
        "prompt": format!("ask this anwser with max 300 characters: {}", prompt),
        "stream": false
    });

    let response = client
        .post(env::var("LLAMA_API_URL").expect("error to get the ia api url from env"))
        .json(&payload)
        .send()
        .await.unwrap();

    let json: IaResponse = response.json().await.unwrap();
    Ok(json.response)
}
