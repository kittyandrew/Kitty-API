// Standard
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::env;
use std::fs;
// Third Party
use rocket_contrib::json::{Json, JsonValue};
// -- "FUCK YOU rust, and rust devs" section --
use dynfmt::{Format, SimpleCurlyFormat};
// -- end of section --
use rand::distributions::Alphanumeric;
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
// use rocket::request::Form;
// use either::Either;
use rocket::State;


// The type to represent id of a user.
pub type ID = usize;
// Storage for all users, instead of DB.
pub type UserMap = Mutex<HashMap<ID, User>>;
// Storage for all profiles
pub type LoginMap = Mutex<HashMap<String, Profile>>;
// Cache for all existing emails
pub type LoginCache = Mutex<HashSet<String>>;


// @FeatureReq: make "Email", "Url" types so we can type check for them (?)
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: ID,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: u8,
    pub active: bool,
    pub picture: String,
}

pub struct Profile {
    pub login: String,
    pub password: String,
    pub session: String,
}

#[derive(Deserialize)]
pub struct Data {
    pub email: String,
    pub password: String,
}

// Context container, useful for configurations
#[derive(Serialize)]
pub struct Context {
    pub page_size: usize,
    pub messages: HashMap<String, String>,
    pub docs_url: String,
}

impl Context {
    pub fn get_message(&self, code: &str) -> &str {
        // TODO: implement translations later
        self.messages.get(code).expect("You done oof-ed with the error messages!").as_str()
    }

    pub fn format_usize(&self, code: &str, args: &Vec<usize>) -> String {
        let message = self.get_message(&code);
        SimpleCurlyFormat.format(message, args).expect("FUCK YOU rust, and rust devs").into_owned()
    }

    pub fn format_str(&self, code: &str, args: &Vec<&str>) -> String {
        let message = self.get_message(&code);
        SimpleCurlyFormat.format(message, args).expect("FUCK YOU rust, and rust devs").into_owned()
    }
}

// Utils

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

pub fn generate_users() -> UserMap {
    let mut map = HashMap::<ID, User>::new();
    // Reading file with users
    let users_raw = fs::read_to_string("./data/users.json")
        .expect("You must provide json file with a list of users!");
    // Parsing array of users
    let mut users_arr: Vec<User> = serde_json::from_str(&users_raw)
        .expect("JSON must be a list of User(s)!");

    for i in 0..users_arr.len() {
        users_arr[i].id = i;
        map.insert(i, users_arr[i].clone());
    }
    UserMap::new(map)
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

