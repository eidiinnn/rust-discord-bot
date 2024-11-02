use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelDetails {
    pub parent_model: String,
    pub format: String,
    pub family: String,
    pub families: Vec<String>,
    pub parameter_size: String,
    pub quantization_level: String,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelInfo {
    architecture: Option<String>,
    file_type: i32,
    parameter_count: u64,
    quantization_version: i32,
    attention_head_count: i32,
    attention_head_count_kv: i32,
    attention_layer_norm_rms_epsilon: f64,
    block_count: i32,
    context_length: i32,
    embedding_length: i32,
    feed_forward_length: i32,
    rope_dimension_count: i32,
    rope_freq_base: u64,
    vocab_size: u64,
    bos_token_id: i32,
    eos_token_id: i32,
    merges: Option<String>,
    model: String,
    pre: String,
    token_type: Option<String>,
    tokens: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IaResponse {
    pub system: Option<String>,
    pub license: Option<String>,
    pub parameters: Option<String>,
    pub template: Option<String>,
    pub details: Option<ModelDetails>,
    pub modified_at: Option<String>,
    pub error: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

pub async fn get_info_from_model(model: &str) -> Result<IaResponse, std::io::Error> {
    let client = Client::new();

    let payload = json!({
        "name" : model,
    });

    let response = client
        .post(format!("{}/show", &crate::common::LLAMA_API_URL))
        .json(&payload)
        .send()
        .await
        .expect("error to get the api response");
    let status = response.status();

    let text_value = response
        .text()
        .await
        .expect("error to get the body in text form");

    let json: IaResponse =
        serde_json::from_str(text_value.as_str()).expect("Error to parse the json");

    if status != StatusCode::OK || json.error.is_some() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request error status={:?} body={:?}", status, json.error),
        ));
    }

    Ok(json)
}

pub async fn create_ai_model(name: &str, model_file: &str) -> Result<(), std::io::Error> {
    let client = Client::new();

    let payload = json!({
       "name": name,
       "modelfile": model_file,
    });

    let request = client
        .post(format!("{}/create", &crate::common::LLAMA_API_URL))
        .json(&payload)
        .send()
        .await
        .expect("Error to create a ia model");
    let status = request.status();

    if status != StatusCode::OK {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            request.text().await.unwrap(),
        ));
    }

    Ok(())
}
