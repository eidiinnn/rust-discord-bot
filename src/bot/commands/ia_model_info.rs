use crate::ia;
use crate::tools::log::CustomLog;
use once_cell::sync::Lazy;
use serenity::all::{ChannelId, Context, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use std::sync::Arc;

static LOG: Lazy<CustomLog> = Lazy::new(|| CustomLog::new(String::from("[Command] [Model Info]")));

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
        LOG.info(format!(
            "receive a model info request model \"{:?}\"",
            &string
        ));
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

        let ia_info = ia::ia_model::get_info_from_model(&model).await.unwrap();

        if ia_info.details.is_none() || ia_info.error.is_some() {
            if ia_info.error.is_some() {
                let _ = channel_id
                    .say(cache_http.http.clone(), ia_info.error.unwrap())
                    .await;
            } else {
                let _ = channel_id
                    .say(cache_http.http.clone(), "unknow error")
                    .await;
            }
            return ();
        }

        let details = ia_info.details.unwrap();

        let text = format!(
            "Parent model: {}\nformat: {}\nfamily: {}\nfamilies: {:?}\nparameter size: {}\nquantization level: {}",
            details.parent_model,
            details.format,
            details.family,
            details.families,
            details.parameter_size,
            details.quantization_level,
        );

        LOG.info(format!("Receive the model info {:?}", &text));
        let _ = channel_id.say(cache_http.http.clone(), text).await;

        typing.stop();
    });
}

pub fn register() -> CreateCommand {
    LOG.info("Register".to_string());
    CreateCommand::new("model")
        .description("get IA model information")
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::String,
            "text",
            "model name",
        ))
}
