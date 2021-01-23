// Standard
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::fs;
// Third Party
use rocket_contrib::json::{Json, JsonValue};
use rand::distributions::Alphanumeric;
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
// use rocket::request::Form;
// use either::Either;
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

#[derive(Deserialize)]
pub struct Data {
    pub email: String,
    pub password: String,
}

// Utils

pub fn reg_data_has_error(data: &Json<Data>, login_cache: &State<LoginCache>) -> Option<JsonValue> {
    match data {
        // Handle empty email
        data if data.email.is_empty() => return Some(json!({ "message": "Email must not be empty!" })),
        // Handle existing account
        data if login_cache.lock().unwrap().contains(&data.email) => return Some(json!({ "message": "User is already registered!" })),
        // Handle empty password
        data if data.password.is_empty() => return Some(json!({ "message": "Password must not be empty!" })),
        // TODO: change function to take arguments for min/max length
        // Handle short password
        data if data.password.len() < 8 => return Some(json!({ "message": "Password is too short! Minimum length is 8 symbols." })),
        // Handle too long password (in case of DDOS)
        data if data.password.len() > 128 => return Some(json!({ "message": "Password is too long! Please use more practical length." })),
        _ => None,
    }
}

pub fn login_data_has_error(data: &Json<Data>, login_cache: &State<LoginCache>) -> Option<JsonValue> {
    match data {
        // Handle empty email
        data if data.email.is_empty() => return Some(json!({ "message": "Email must not be empty!" })),
        // Handle existing account
        data if !login_cache.lock().unwrap().contains(&data.email) => return Some(json!({ "message": "Account with such email does not exist!" })),
        // Handle empty password
        data if data.password.is_empty() => return Some(json!({ "message": "Password must not be empty!" })),
        // Handle invalid entries for password
        // Note: while handling those entries - just say they are incorrect (same error as non-existent account)
        data if (data.password.len() < 8) | (data.password.len() > 128) => return Some(json!({ "message": "Password was incorrect!" })),
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
