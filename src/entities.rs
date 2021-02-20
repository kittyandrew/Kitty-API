use std::collections::{HashMap, HashSet};
use rocket_contrib::databases::postgres;
// -- "FUCK YOU rust, and rust devs" section --
use dynfmt::{Format, SimpleCurlyFormat};
// -- end of section --
use serde::{Serialize, Deserialize};
use std::sync::Mutex;


// The type to represent id of a user.
pub type ID = u32;
// Storage for all profiles
pub type LoginMap = Mutex<HashMap<String, Profile>>;
// Cache for all existing emails
pub type LoginCache = Mutex<HashSet<String>>;

#[database("kittybox")]
pub struct KittyBox(postgres::Client);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: ID,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub age: u32,
    pub active: bool,
    pub picture: String,
}

impl User {
    pub fn from_row(row: &postgres::Row) -> User {
        let id: i32 = row.get("id");
        let age: i32 = row.get("age");
        User {
            id:         id as u32,  // Explicit conversion
            username:   row.get("username"),
            first_name: row.get("first_name"),
            last_name:  row.get("last_name"),
            email:      row.get("email"),
            age:        age as u32,  // Explicit conversion
            active:     row.get("active"),
            picture:    row.get("picture"),
        }
    }

    // Insert and auto-assign (auto-increment) id
    pub fn insert(&self, c: &mut postgres::Client) -> i32 {
        c.query_one(
            "INSERT INTO users \
             (username, first_name, last_name, email, age, active, picture) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) \
             RETURNING id",
            &[
                &self.username, &self.first_name, &self.last_name,
                &self.email, &(self.age as i32), &self.active, &self.picture,
            ],
        ).unwrap().get("id")
    }

    // Insert with certain id
    pub fn insert_with_id(&self, c: &mut postgres::Client) -> Result<i32, postgres::Error> {
        //     @: We insert into all columns so can remove this
        //     (id, username, first_name, last_name, email, age, active, picture) \
        match c.query_one(
            "INSERT INTO users \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
             RETURNING id",
            &[
                &(self.id as i32), &self.username, &self.first_name, &self.last_name,
                &self.email, &(self.age as i32), &self.active, &self.picture,
            ],
        ) {
            Ok(item) => Ok(item.get("id")),
            Err(e) => Err(e),
        }
    }

    // Insert or update, depending whether id exists
    pub fn put(&self, c: &mut postgres::Client) -> i32 {
        c.query_one(
            "INSERT INTO users \
             (id, username, first_name, last_name, email, age, active, picture) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
             ON CONFLICT (id) DO UPDATE SET \
             username = EXCLUDED.username, first_name = EXCLUDED.first_name, \
             last_name = EXCLUDED.last_name, email = EXCLUDED.email, \
             age = EXCLUDED.age, active = EXCLUDED.active, picture = EXCLUDED.picture \
             RETURNING id",
            &[
                &(self.id as i32), &self.username, &self.first_name, &self.last_name,
                &self.email, &(self.age as i32), &self.active, &self.picture,
            ],
        ).unwrap().get("id")
    }
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

