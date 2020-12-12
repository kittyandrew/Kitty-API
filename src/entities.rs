// Standard
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::fs;
// Third Party
use rand::distributions::Alphanumeric;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;
use chrono::{DateTime, Utc};
use rand::{thread_rng, Rng};
use rocket::State;
// The type to represent id of a user.
pub type ID = usize;

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

// Storage for all users, instead of DB.
pub type UserMap = Mutex<HashMap<ID, User>>;
// Storage for all profiles
pub type LoginMap = Mutex<HashMap<String, Profile>>;
// Cache for all existing emails
pub type LoginCache = Mutex<HashSet<String>>;

// page object for some amount of users
#[derive(Serialize)]
pub struct UserPage {
    pub page: usize,
    pub per_page: usize,
    pub items: usize,
    pub next_exist: bool,
    pub data: Vec<User>,
}

#[derive(Deserialize)]
pub struct Data {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct GoodRegResp {
    pub message: String,
    pub session: String,
    pub creation_date: DateTime<Utc>,
}

#[derive(Serialize, Responder)]
pub struct BadRegResp {
    pub message: String,
}

#[derive(Responder)]
pub enum AnyResp {
    Good(Json<GoodRegResp>),
    GoodAll(Json<Vec<User>>),
    #[response(status = 401, content_type = "json")]
    Bad(Json<BadRegResp>),
}

// Utils

pub fn reg_data_has_error(data: &Json<Data>, login_cache: &State<LoginCache>) -> Option<AnyResp> {
    let cache = login_cache.lock().unwrap();
    match data {
        // Handle empty email
        data if data.email.is_empty() => return Some(AnyResp::Bad(Json(BadRegResp { message: "Email must not be empty!".to_string() }))),
        // Handle existing account
        data if cache.contains(&data.email) => return Some(AnyResp::Bad(Json(BadRegResp { message: "User is already registered!".to_string() }))),
        // Handle empty password
        data if data.password.is_empty() => return Some(AnyResp::Bad(Json(BadRegResp { message: "Password must not be empty!".to_string() }))),
        // Handle short password
        data if data.password.len() < 8 => return Some(AnyResp::Bad(Json(BadRegResp { message: "Password is too short! Minimum length is 8 symbols.".to_string() }))),
        // Handle too long password (in case of DDOS)
        data if data.password.len() > 128 => return Some(AnyResp::Bad(Json(BadRegResp { message: "Password is too long! Please use more practical length.".to_string() }))),
        _ => None,
    }
}

pub fn login_data_has_error(data: &Json<Data>, login_cache: &State<LoginCache>) -> Option<AnyResp> {
    let cache = login_cache.lock().unwrap();
    match data {
        // Handle empty email
        data if data.email.is_empty() => return Some(AnyResp::Bad(Json(BadRegResp { message: "Email must not be empty!".to_string() }))),
        // Handle existing account
        data if !cache.contains(&data.email) => return Some(AnyResp::Bad(Json(BadRegResp { message: "Account with such email does not exist!".to_string() }))),
        // Handle empty password
        data if data.password.is_empty() => return Some(AnyResp::Bad(Json(BadRegResp { message: "Password must not be empty!".to_string() }))),
        // Handle invalid entries for password
        // Note: while handling those entries - just say they are incorrect (same error as non-existent account)
        data if (data.password.len() < 8) | (data.password.len() > 128) => return Some(AnyResp::Bad(Json(BadRegResp { message: "Password was incorrect!".to_string() }))),
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
    Mutex::new(map)
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
