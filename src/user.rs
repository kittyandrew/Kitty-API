// Standard
use std::collections::HashMap;
use std::sync::Mutex;
// Third Party
use serde::Serialize;

// The type to represent id of a user.
pub type ID = usize;

// @FeatureReq: make "Email", "Url" types so we can type check for them (?)
#[derive(Serialize, Copy)]
pub struct User {
    pub id: u8,
    pub username: &'static str,
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub email: &'static str,
    pub age: u8,
    pub active: bool,
    pub picture: &'static str,
}

impl Clone for User {
    fn clone(&self) -> User {
        *self
    }
}

// Storage for all users, instead of DB.
pub type UserMap = Mutex<HashMap<ID, User>>;

// Utils

pub fn generate_users() -> UserMap {
    let mut map = HashMap::<ID, User>::new();
    // Note: inserting data in reverse order to keep it naturally sorted from 1 to n
    map.insert(4, User {
        id: 4,
    	username: "sunday_lover",
    	first_name: "Mari",
    	last_name: "York",
    	email: "mariyork1997@gmail.com",
    	age: 23,
    	active: true,
    	picture: "https://www.stockvault.net/data/2012/09/01/134826/preview16.jpg",
    });
    map.insert(3, User {
        id: 3,
    	username: "juliochapman50",
    	first_name: "Douglas",
    	last_name: "Mac",
    	email: "juliothegiulio@gmail.com",
    	age: 61,
    	active: true,
    	picture: "https://www.stockvault.net/data/2019/03/18/262499/preview16.jpg",
    });
    map.insert(2, User {
        id: 2,
    	username: "anotherusername666",
    	first_name: "That",
    	last_name: "Guy",
    	email: "hello@thatguy.me",
    	age: 19,
    	active: false,
    	picture: "https://google.com/search?q=Stan%20for%20comma.ai&tbm=isch",
    });
    map.insert(1, User {
        id: 1,
        username: "someusername123",
    	first_name: "Someone",
    	last_name: "Fancy",
    	email: "someone@example.com",
    	age: 44,
    	active: true,
    	picture: "https://google.com/search?q=Picture%20Of%20Some%20Fancy%20Man",
    });
    Mutex::new(map)
}
