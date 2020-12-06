// Standard
use std::collections::HashMap;
use std::sync::Mutex;
use std::fs;
// Third Party
use serde::{Serialize, Deserialize};
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

// Storage for all users, instead of DB.
pub type UserMap = Mutex<HashMap<ID, User>>;

// page object for some amount of users
#[derive(Serialize)]
pub struct UserPage {
    pub page: usize,
    pub page_size: usize,
    pub returned_size: usize,
    pub next_exist: bool,
    pub data: Vec<User>,
}

// Utils

pub fn generate_users() -> UserMap {
    let mut map = HashMap::<ID, User>::new();
    // Reading file with users
    let users_raw = fs::read_to_string("./data/users.json")
	.expect("You must provide json file with a list of users!");
    // Parsing array of users
    let mut users_arr: Vec<User> = serde_json::from_str(&users_raw)
	.expect("JSON must be a list of User(s)!");

    for i in 0..users_arr.len() {
	users_arr[i].id = i + 1;
	map.insert(i + 1, users_arr[i].clone());
    }
    Mutex::new(map)
}
