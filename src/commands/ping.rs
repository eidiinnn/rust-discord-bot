use crate::tools::log::CustomLog;
use once_cell::sync::Lazy;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

static LOG: Lazy<CustomLog> = Lazy::new(|| CustomLog::new(String::from("[Command] [Ping Pong]")));

pub fn run(_options: &[ResolvedOption]) -> String {
    LOG.info("Receive a request".to_string());
    "pong".to_string()
}

pub fn register() -> CreateCommand {
    LOG.info("Register".to_string());
    CreateCommand::new("ping").description("A ping command")
}
