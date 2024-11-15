use once_cell::sync::Lazy;
use redis::Client;

pub static REDIS_CONNECTION: Lazy<Client> =
    Lazy::new(|| Client::open(crate::common::REDIS_URL).unwrap());
