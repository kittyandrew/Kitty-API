// Standard
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::env;
use std::fs;
// Third Party
use rocket_contrib::json::{Json, JsonValue};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
// use rocket::request::Form;
// use either::Either;
use rocket::State;
use crate::entities::{
    ID, User, UserMap, Context, LoginCache, Data,
    LoginMap, Profile,
};

pub fn reg_data_has_error(data: &Json<Data>, login_cache: &State<LoginCache>, context: &State<Context>) -> Option<JsonValue> {
    match data {
        // Handle empty email
        data if data.email.is_empty() => return Some(json!({
            "msg_code": "err_email_empty",
            "message": context.get_message("err_email_empty")
        })),
        // Handle empty password
        data if data.password.is_empty() => return Some(json!({
            "msg_code": "err_password_empty",
            "message": context.get_message("err_password_empty")
        })),
        // TODO: change function to take arguments for min/max length
        // Handle short password
        data if data.password.len() < 8 => return Some(json!({
            "msg_code": "err_password_short",
            "message": context.get_message("err_password_short")
        })),
        // Handle too long password (avoid DDOS)
        data if data.password.len() > 128 => return Some(json!({
            "msg_code": "err_password_long",
            "message": context.get_message("err_password_long")
        })),
        // Handle existing account
        data if login_cache.lock().unwrap().contains(&data.email) => return Some(json!({
            "msg_code": "err_email_taken",
            "message": context.format_str("err_email_taken", &vec![&data.email])
        })),
        // No err
        _ => None,
    }
}

pub fn login_data_has_error(data: &Json<Data>, login_cache: &State<LoginCache>, context: &State<Context>) -> Option<JsonValue> {
    match data {
        // Handle empty email
        data if data.email.is_empty() => return Some(json!({
            "msg_code": "err_email_empty",
            "message": context.get_message("err_email_empty")
        })),
        // Handle empty password
        data if data.password.is_empty() => return Some(json!({
            "msg_code": "err_password_empty",
            "message": context.get_message("err_password_empty")
        })),
        // TODO: change function to take arguments for min/max length
        // Handle short password
        data if data.password.len() < 8 => return Some(json!({
            "msg_code": "err_password_short",
            "message": context.get_message("err_password_short")
        })),
        // Handle too long password (avoid DDOS)
        data if data.password.len() > 128 => return Some(json!({
            "msg_code": "err_password_long",
            "message": context.get_message("err_password_long")
        })),
        // Handle existing account
        data if !login_cache.lock().unwrap().contains(&data.email) => return Some(json!({
            "msg_code": "err_bad_credentials",
            "message": context.get_message("err_bad_credentials")
        })),
        // No err
        _ => None,
    }
}

pub fn get_free_index<T>(map: &HashMap<usize, T>) -> usize {
    let mut top_key: usize = 0;
    for key in map.keys() {
        if key > &top_key { top_key = *key; }
    }
    let mut index: usize = 1;
    // Now looking for smallest free index to add (in the worst case, we will append in the end)
    for i in 0..top_key + 1 {
        index = i + 1;
        if !map.contains_key(&index) { break }
    }
    index
}

pub fn generate_users() -> Vec<User> {
    // Reading file with users
    let users_raw = fs::read_to_string("./data/users.json")
        .expect("You must provide json file with a list of users!");
    // Parsing array of users
    let mut users_arr: Vec<User> = serde_json::from_str(&users_raw)
        .expect("JSON must be a list of User(s)!");
    // return
    users_arr
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
    // Load error codes and messages
    let raw_status_messages = fs::read_to_string("./data/status_messages.json")
        .expect("You forgot to provide json file with status messages!");
    let status_messages: HashMap<String, String> = serde_json::from_str(&raw_status_messages)
        .expect("Failed to parse 'status_messages'! rip.");

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
        messages: status_messages.clone(),
        docs_url: "https://github.com/kittyandrew/Kitty-API/tree/master/docs/DOCS.md".to_string(),
    }
}

