use crate::ia_api;
use serenity::all::{ChannelId, Context, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::sync::Arc;

pub async fn run(
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
    context: &Context,
) -> String {
    let mut model: Option<String> = None;
    if let Some(ResolvedOption {
        value: ResolvedValue::String(string),
        ..
    }) = options.get(1)
    {
        model = Some(string.to_string())
    }

    if let Some(ResolvedOption {
        value: ResolvedValue::String(string),
        ..
    }) = options.get(0)
    {
        say_with_ia_response(
            string.to_string(),
            channel_id.into(),
            context.clone().into(),
            model,
        );
        return format!("processing the message \"{}\"", string.to_string());
    } else {
        return "empty text".to_string();
    }
}

fn say_with_ia_response(
    question: String,
    channel_id: Arc<ChannelId>,
    cache_http: Arc<Context>,
    model: Option<String>,
) {
    tokio::spawn(async move {
        let typing = channel_id.start_typing(&cache_http.http.clone());

        let model: String = match model {
            Some(model) => model,
            None => (&crate::common::NORMAL_LLAMA_MODEL).to_string(),
        };

        let ia_reponse = ia_api::ia_ask::ask(question, model, None)
            .await
            .unwrap();
        let _ = channel_id.say(cache_http.http.clone(), ia_reponse).await;
        typing.stop();
    });
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ask")
        .description("Ask for ia")
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::String,
            "text",
            "text",
        ))
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::String,
            "model",
            "choose a model to use",
        ))
}
