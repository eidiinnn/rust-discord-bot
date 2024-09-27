use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serenity::futures::StreamExt;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct IaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

pub async fn ia_ask(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let model = env::var("LLAMA_MODEL").expect("error to get the ia model");

    let payload = json!({
        "model": model,
        "prompt": prompt,
    });

    // Declare response as mutable
    let mut response = client
        .post(env::var("LLAMA_API_URL").expect("error to get the ia api!"))
        .json(&payload)
        .send()
        .await?
        .bytes_stream();

    let mut full_response = String::new();

    while let Some(chunk) = response.next().await {
        let chunk = chunk?;
        let ia_response: IaResponse = serde_json::from_slice(&chunk)?;

        full_response.push_str(&ia_response.response);
        if ia_response.done {
            break;
        }
    }

    println!("response {:?}", full_response);

    Ok(full_response)
}
