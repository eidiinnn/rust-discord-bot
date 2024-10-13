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
    if let Some(ResolvedOption {
        value: ResolvedValue::String(string),
        ..
    }) = options.first()
    {
        say_with_ia_response(
            string.to_string(),
            channel_id.into(),
            context.clone().into(),
        );
        return format!("Looking for model: \"{}\"", string.to_string());
    } else {
        return "empty text".to_string();
    }
}

fn say_with_ia_response(model_name: String, channel_id: Arc<ChannelId>, cache_http: Arc<Context>) {
    tokio::spawn(async move {
        let typing = channel_id.start_typing(&cache_http.http.clone());
        let model = &model_name.as_str();

        let ia_info = ia_api::ia_model::get_info_from_model(&model).await.unwrap();

        let text = format!(
            "Parent model: {}\nformat: {}\nfamily: {}\nfamilies: {:?}\nparameter size: {}\nquantization level: {}",
            ia_info.details.parent_model,
            ia_info.details.format,
            ia_info.details.family,
            ia_info.details.families,
            ia_info.details.parameter_size,
            ia_info.details.quantization_level,
        );

        let _ = channel_id.say(cache_http.http.clone(), text).await;

        typing.stop();
    });
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ia_model_info")
        .description("get ia model details")
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::String,
            "text",
            "model name",
        ))
}
