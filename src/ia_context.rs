use crate::redis::REDIS_CONNECTION;
use redis;
use serde_json;
use std::error::Error;

pub type RedisResult<T> = Result<T, Box<dyn Error>>;

pub fn save_context(user_id: String, context: Vec<i32>) -> RedisResult<()> {
    let json_string = serde_json::to_string(&context)?;
    let mut connection = REDIS_CONNECTION.get_connection()?;

    redis::cmd("SET")
        .arg(user_id)
        .arg(json_string)
        .exec(&mut connection)?;

    Ok(())
}

pub fn get_context(user_id: String) -> RedisResult<Option<Vec<i32>>> {
    let mut connection = REDIS_CONNECTION.get_connection()?;

    let query: Option<String> = redis::cmd("GET").arg(user_id).query(&mut connection)?;
   
    match query {
        Some(query) => {
           let context: Vec<i32> = serde_json::from_str(&query)?;
           Ok(Some(context))
        },
        None => Ok(None),
    }

}
