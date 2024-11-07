mod commands;
mod common;
mod ia_api;
mod tools;
use common::{RPG_LLAMA_MODEL, RPG_LLAMA_MODEL_FILE};
use dotenv::dotenv;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage, GuildId, Ready};
use serenity::async_trait;
use serenity::model::application::Interaction;
use serenity::prelude::*;
use std::env;
use tools::log::CustomLog;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "ask" => Some(
                    commands::ia_ask::run(&command.data.options(), command.channel_id, &ctx).await,
                ),
                "ia_model_info" => Some(
                    commands::ia_model_info::run(&command.data.options(), command.channel_id, &ctx)
                        .await,
                ),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _ = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::ping::register(),
                    commands::ia_ask::register(),
                    commands::ia_model_info::register(),
                ],
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    let log = CustomLog::new(String::from("[Discord Bot] [Setup]"));
    setup_rpg_ia_model().await;

    dotenv().ok();
    let token = env::var("TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    log.info(String::from("Connecting on discord bot..."));
    match client.start().await {
        Ok(_) => log.info(String::from("Bot connected!")),
        Err(why) => log.error(format!("Client error: {why:?}")),
    }
}

async fn setup_rpg_ia_model() {
    let log = CustomLog::new(String::from("[RPG IA] [Setup]"));

    match ia_api::ia_model::create_ai_model(&RPG_LLAMA_MODEL, &RPG_LLAMA_MODEL_FILE).await {
        Ok(_) => log.info(String::from("RPG IA Created")),
        Err(error) => {
            let error = format!("Error to define the IA {:?}", error);
            log.error(String::from(error));
            panic!();
        }
    }
}
