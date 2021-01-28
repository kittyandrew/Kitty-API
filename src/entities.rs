// Standard
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
// Third Party
// -- "FUCK YOU rust, and rust devs" section --
use dynfmt::{Format, SimpleCurlyFormat};
// -- end of section --
use serde::{Serialize, Deserialize};


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

