use crate::ia::ia_ask::IaResponse;
use crate::ia::ia_context::{delete_context, get_context, save_context};
use crate::tools::log::CustomLog;
use once_cell::sync::Lazy;
use serenity::all::{ChannelId, Context, GetMessages, MessageId, User, UserId};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

static LOG: Lazy<CustomLog> = Lazy::new(|| CustomLog::new(String::from("[Command] [IA RPG]")));

pub async fn run(
    options: &Vec<ResolvedOption<'_>>,
    channel_id: ChannelId,
    user_id: User,
    context: &Context,
) -> String {
    start_rpg_section(
        channel_id.into(),
        user_id.clone().into(),
        context.clone().into(),
    );
    return format!("RPG starting...");
}

fn start_rpg_section(channel_id: ChannelId, user_id: User, cache_http: Context) {
    tokio::spawn(async move {
        let rpg_theme_choose_msg = channel_id
            .say(
                cache_http.http.clone(),
                String::from("What is the RPG theme? (use /exit to end the session)"),
            )
            .await
            .unwrap();

        let mut last_id: MessageId = rpg_theme_choose_msg.id;
        let http = &cache_http.http;

        loop {
            let message_filter = GetMessages::new().after(last_id).limit(2);

            let messages = channel_id
                .messages(&http.clone(), message_filter)
                .await
                .unwrap();

            if messages.len() > 0
                && messages[0].author == user_id
                && messages[0].content != "/exit".to_string()
            {
                let id = make_prompt(
                    messages[0].content.clone(),
                    channel_id,
                    user_id.clone().into(),
                    cache_http.clone(),
                )
                .await;
                last_id = id;
            };

            if messages.len() > 0 && messages[0].content == "/exit".to_string() {
                let _ = delete_context(
                    &user_id.id.to_string(),
                    Some(&crate::common::RPG_LLAMA_MODEL.to_string()),
                )
                .unwrap();

                channel_id
                    .say(cache_http.http.clone(), String::from("Session deleted!"))
                    .await
                    .unwrap();

                break;
            }
        }
    });
}

async fn make_prompt(
    action: String,
    channel_id: ChannelId,
    user_id: UserId,
    cache_http: Context,
) -> MessageId {
    let typing = channel_id.start_typing(&cache_http.http.clone());
    let user_id = user_id.to_string();

    let model: String = (&crate::common::RPG_LLAMA_MODEL).to_string();

    let context = get_context(&user_id, Some(&model)).unwrap();

    let ia_response: IaResponse = crate::ia::ia_ask::ask(action, model.clone(), context)
        .await
        .unwrap();

    let _ = save_context(&user_id, Some(&model), &ia_response.context).unwrap();

    LOG.info(format!(
        "receive the response from ia \"{:?}\"",
        ia_response.response
    ));
    let response = channel_id
        .say(cache_http.http.clone(), ia_response.response)
        .await
        .unwrap();
    typing.stop();
    return response.id;
}

pub fn register() -> CreateCommand {
    LOG.info("Register".to_string());
    CreateCommand::new("play_rpg").description("Start a text RPG journey")
}
