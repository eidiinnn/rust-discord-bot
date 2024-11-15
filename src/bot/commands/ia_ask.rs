use crate::ia::ia_ask::IaResponse;
use crate::ia::ia_context::{get_context, save_context};
use crate::tools::log::CustomLog;
use once_cell::sync::Lazy;
use serenity::all::{ChannelId, Context, CreateCommandOption, ResolvedValue, UserId};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

static LOG: Lazy<CustomLog> = Lazy::new(|| CustomLog::new(String::from("[Command] [IA Ask]")));

pub async fn run(
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
    user_id: UserId,
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
        LOG.info(format!(
            "Receive a IA ask request with the params text=\"{:?}\" model=\"{:?}\"",
            &string, &model
        ));
        say_with_ia_response(
            string.to_string(),
            channel_id.into(),
            user_id.clone().into(),
            context.clone().into(),
            model,
        );
        return format!("processing the message \"{}\"", string.to_string());
    } else {
        LOG.info("empty text receive".to_string());
        return "empty text".to_string();
    }
}

fn say_with_ia_response(
    question: String,
    channel_id: ChannelId,
    user_id: UserId,
    cache_http: Context,
    model: Option<String>,
) {
    tokio::spawn(async move {
        let typing = channel_id.start_typing(&cache_http.http.clone());
        let user_id = user_id.to_string();

        let model: String = match model {
            Some(model) => model,
            None => (&crate::common::NORMAL_LLAMA_MODEL).to_string(),
        };

        let context = get_context(&user_id, Some(&model)).unwrap();

        let ia_response: IaResponse = crate::ia::ia_ask::ask(question, model.clone(), context)
            .await
            .unwrap();

        let _ = save_context(
            &user_id,
            Some(&model),
            &ia_response.context,
        )
        .unwrap();

        LOG.info(format!(
            "receive the response from ia \"{:?}\"",
            ia_response.response
        ));
        let _ = channel_id
            .say(cache_http.http.clone(), ia_response.response)
            .await;
        typing.stop();
    });
}

pub fn register() -> CreateCommand {
    LOG.info("Register".to_string());
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
