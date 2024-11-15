mod bot;
mod common;
mod ia;
mod redis;
mod tools;
use common::{RPG_LLAMA_MODEL, RPG_LLAMA_MODEL_FILE};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use tools::log::CustomLog;

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_rpg_ia_model().await;
    bot::bot_manager::setup().await;
}

async fn setup_rpg_ia_model() {
    let setup_ia_log: Lazy<CustomLog> = Lazy::new(|| CustomLog::new(String::from("[IA] [Setup]")));
    match ia::ia_model::create_ai_model(&RPG_LLAMA_MODEL, &RPG_LLAMA_MODEL_FILE).await {
        Ok(_) => setup_ia_log.info(String::from("RPG IA Created")),
        Err(error) => {
            let error = format!("Error to define the IA {:?}", error);
            setup_ia_log.error(String::from(error));
            panic!();
        }
    }
}
