use crate::{common::NORMAL_LLAMA_MODEL, redis::REDIS_CONNECTION};
use redis;
use serde_json;
use std::error::Error;

pub type RedisResult<T> = Result<T, Box<dyn Error>>;

pub fn save_context(
    user_id: &String,
    model: Option<&String>,
    context: &Vec<i32>,
) -> RedisResult<()> {
    let json_string = serde_json::to_string(&context)?;
    let mut connection = REDIS_CONNECTION.get_connection()?;

    redis::cmd("SET")
        .arg(get_context_db_name(user_id, model))
        .arg(json_string)
        .exec(&mut connection)?;

    Ok(())
}

pub fn get_context(user_id: &String, model: Option<&String>) -> RedisResult<Option<Vec<i32>>> {
    let mut connection = REDIS_CONNECTION.get_connection()?;

    let query: Option<String> = redis::cmd("GET")
        .arg(get_context_db_name(user_id, model))
        .query(&mut connection)?;

    match query {
        Some(query) => {
            let context: Vec<i32> = serde_json::from_str(&query)?;
            Ok(Some(context))
        }
        None => Ok(None),
    }
}

pub fn delete_context(user_id: &String, model: Option<&String>) -> RedisResult<()> {
    let mut connection = REDIS_CONNECTION.get_connection()?;

    redis::cmd("DEL")
        .arg(get_context_db_name(user_id, model))
        .exec(&mut connection)?;

    Ok(())
}

fn get_context_db_name(user_id: &String, model: Option<&String>) -> String {
    let model: String = match model {
        Some(value) => value.to_string(),
        None => NORMAL_LLAMA_MODEL.to_owned(),
    };

    return format!("{}-{}", user_id, model);
}
