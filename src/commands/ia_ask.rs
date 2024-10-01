use std::sync::Arc;

use crate::ia_api;
use serenity::all::{ChannelId, Context, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub async fn run(
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
    context: &Context,
) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(string),
        ..
    }) = options.first()
    {
        say_with_ia_response(string.to_string(), channel_id.into(), context.clone().into());
        return format!("processing the message \"{}\"", string.to_string());
    } else {
        return "empty text".to_string();
    }
}

fn say_with_ia_response(question: String, channel_id: Arc<ChannelId>, cache_http: Arc<Context>) {
    tokio::spawn(async move {
      let typing = channel_id.start_typing(&cache_http.http.clone());
      let ia_reponse = ia_api::ia_ask(question).await.unwrap();
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
}
