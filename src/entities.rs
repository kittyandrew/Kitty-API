use rocket_contrib::{databases::postgres, json::{Json, JsonValue}};
use rocket::http::{Header, Status, hyper::header::ACCEPT};
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use data_item::{DataItem, KittyBox};
use rocket::{Route, Response};
use crate::headers::PageSize;
use std::sync::Mutex;


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
    pub docs_url: String,
}

