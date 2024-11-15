use crate::tools::log::CustomLog;
use once_cell::sync::Lazy;
use serenity::all::{CreateCommandOption, ResolvedOption, ResolvedValue, UserId};
use serenity::builder::CreateCommand;

static LOG: Lazy<CustomLog> =
    Lazy::new(|| CustomLog::new(String::from("[Command] [Delete context]")));

pub fn run(options: &Vec<ResolvedOption<'_>>, user_id: UserId) -> String {
    let user_id = user_id.to_string();
    let mut model: Option<String> = None;

    if let Some(ResolvedOption {
        value: ResolvedValue::String(string),
        ..
    }) = options.get(0)
    {
        model = Some(string.to_string())
    }

    LOG.info(format!(
        "Deleting the context from user={:?} model={:?}",
        user_id.to_string(),
        &model
    ));
    delete_context_on_database(user_id, model);
    return String::from("Context deleted");
}

fn delete_context_on_database(user_id: String, model: Option<String>) {
    tokio::spawn(async move {
        crate::ia::ia_context::delete_context(&user_id, model.as_ref()).unwrap()
    });
}

pub fn register() -> CreateCommand {
    LOG.info("Register".to_string());
    CreateCommand::new("delete_context")
        .description("delete a IA context")
        .add_option(CreateCommandOption::new(
            serenity::all::CommandOptionType::String,
            "model",
            "choose a model to use",
        ))
}
