use rocket_contrib::json::{Json, JsonValue};
use std::collections::{HashMap, HashSet};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::State;
use std::env;
// Own code
use crate::entities::{
    LoginCache, Data, LoginMap, Profile, Context,
};


pub fn reg_data_has_error(data: &Json<Data>, login_cache: &State<LoginCache>) -> Option<JsonValue> {
    match data {
        // Handle empty email
        data if data.email.is_empty() => return Some(json!({
            "msg_code": "err_email_empty",
            // "message": context.get_message("err_email_empty")
        })),
        // Handle empty password
        data if data.password.is_empty() => return Some(json!({
            "msg_code": "err_password_empty",
            // "message": context.get_message("err_password_empty")
        })),
        // TODO: change function to take arguments for min/max length
        // Handle short password
        data if data.password.len() < 8 => return Some(json!({
            "msg_code": "err_password_short",
            // "message": context.get_message("err_password_short")
        })),
        // Handle too long password (avoid DDOS)
        data if data.password.len() > 128 => return Some(json!({
            "msg_code": "err_password_long",
            // "message": context.get_message("err_password_long")
        })),
        // Handle existing account
        data if login_cache.lock().unwrap().contains(&data.email) => return Some(json!({
            "msg_code": "err_email_taken",
            // "message": context.format_str("err_email_taken", &vec![&data.email])
        })),
        // No err
        _ => None,
    }
}


pub fn login_data_has_error(data: &Json<Data>, login_cache: &State<LoginCache>) -> Option<JsonValue> {
    match data {
        // Handle empty email
        data if data.email.is_empty() => return Some(json!({
            "msg_code": "err_email_empty",
            // "message": context.get_message("err_email_empty")
        })),
        // Handle empty password
        data if data.password.is_empty() => return Some(json!({
            "msg_code": "err_password_empty",
            // "message": context.get_message("err_password_empty")
        })),
        // TODO: change function to take arguments for min/max length
        // Handle short password
        data if data.password.len() < 8 => return Some(json!({
            "msg_code": "err_password_short",
            // "message": context.get_message("err_password_short")
        })),
        // Handle too long password (avoid DDOS)
        data if data.password.len() > 128 => return Some(json!({
            "msg_code": "err_password_long",
            // "message": context.get_message("err_password_long")
        })),
        // Handle existing account
        data if !login_cache.lock().unwrap().contains(&data.email) => return Some(json!({
            "msg_code": "err_bad_credentials",
            // "message": context.get_message("err_bad_credentials")
        })),
        // No err
        _ => None,
    }
}


pub fn get_login_storage() -> LoginMap {
    LoginMap::new(HashMap::<String, Profile>::new())
}


pub fn get_login_cache() -> LoginCache {
    LoginCache::new(HashSet::<String>::new())
}


pub fn new_session(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
                .take(n)
                .collect()
}


pub fn get_context() -> Context {
    Context {
        page_size: match env::var("PAGE_SIZE") {
            Ok(val) => match val.parse::<usize>() {
                Ok(num) => num,
                // default value
                Err(_) => 5,
            },
            // default value
            Err(_) => 5,
        },
        docs_url: "https://github.com/kittyandrew/Kitty-API/tree/master/docs/DOCS.md".to_string(),
    }
}

