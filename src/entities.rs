use rocket_contrib::json::{Json, JsonValue};
use std::collections::{HashMap, HashSet};
use dynfmt::{Format, SimpleCurlyFormat};
use rocket_contrib::databases::postgres;
use serde::{Serialize, Deserialize};
use data_item::{DataItem, KittyBox};
use crate::headers::PageSize;
use std::sync::Mutex;
use rocket::Route;


// Storage for all profiles
pub type LoginMap = Mutex<HashMap<String, Profile>>;
// Cache for all existing emails
pub type LoginCache = Mutex<HashSet<String>>;


#[derive(Serialize, Deserialize, Clone, Debug, DataItem)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: u32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: u32,
    pub active: bool,
    pub picture: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, DataItem)]
pub struct Cat {
    #[serde(skip_deserializing)]
    pub id: u32,
    pub name: String,
    pub breed: String,
    pub age: u32,
    // SI units: grams
    pub weight: u32,
    pub picture: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, DataItem)]
pub struct TextCat {
    #[serde(skip_deserializing)]
    pub id: u32,
    pub text: String,
    pub is_ascii: bool,
    pub length: u32,
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

    pub fn format_str(&self, code: &str, args: &Vec<&str>) -> String {
        let message = self.get_message(&code);
        SimpleCurlyFormat.format(message, args).expect("FUCK YOU rust, and rust devs").into_owned()
    }
}

