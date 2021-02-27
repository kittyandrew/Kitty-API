use rocket::figment::{Figment, providers::{Format, Toml}};
use postgres::{Client, NoTls};
use std::fs;
// Own code
use crate::entities::{User, Cat, TextCat};


pub fn generate_users() -> Vec<User> {
    // Reading file with users
    let users_raw = fs::read_to_string("./data/users.json")
        .expect("You must provide json file with a list of users!");
    // Parsing array of users
    serde_json::from_str(&users_raw)
        .expect("JSON must be a list of User(s)!")
}


pub fn generate_cats() -> Vec<Cat> {
    // Read file with cats
    let cats_raw = fs::read_to_string("./data/cats.json")
        .expect("You must provide json file with a list of cats!");
    // Parsing array of users
    let mut cats: Vec<Cat> = serde_json::from_str(&cats_raw)
        .expect("JSON must be a list of Cat(s)!");
    for cat in &mut cats {
        cat.picture = format!("/static/pics/cats/256x256/{}.jpg", &cat.name.to_lowercase());
    }
    cats
}


pub fn generate_textcats() -> Vec<TextCat> {
    // Reading file with users
    let cats_raw = fs::read_to_string("./data/textcats.json")
        .expect("You must provide json file with a list of users!");
    // Parsing array of users
    serde_json::from_str(&cats_raw)
        .expect("JSON must be a list of TextCat(s)!")
}


pub fn load_data() {
    let f = Figment::new()
        .merge(Toml::file("Rocket.toml"));

    let db_url: String;
    if cfg!(debug_assertions) {
        db_url = f.extract_inner("default.databases.kittybox.url").unwrap();
    } else {
        db_url = f.extract_inner("release.databases.kittybox.url").unwrap();
    }
    let mut client = Client::connect(&db_url, NoTls).unwrap();
    // First wipe out the table, and restart ID count
    client.execute("TRUNCATE TABLE users RESTART IDENTITY", &[]).unwrap();
    client.execute("TRUNCATE TABLE cats RESTART IDENTITY", &[]).unwrap();
    client.execute("TRUNCATE TABLE textcats RESTART IDENTITY", &[]).unwrap();

    for user in generate_users() {
        client.execute(
            "INSERT INTO users \
            (username, first_name, last_name, email, age, active, picture) \
            VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &user.username, &user.first_name, &user.last_name,
                &user.email, &(user.age as i32), &user.active, &user.picture,
            ],
        ).expect(&format!("Couldn't add user '{:?}' to database!", user));
    }
    for cat in generate_cats() {
        client.execute(
            "INSERT INTO cats \
            (name, breed, age, weight, picture) \
            VALUES ($1, $2, $3, $4, $5)",
            &[
                &cat.name, &cat.breed, &(cat.age as i32),
                &(cat.weight as i32), &cat.picture,
            ],
        ).expect(&format!("Couldn't add cat '{:?}' to database!", cat));
    }
    for textcat in generate_textcats() {
        client.execute(
            "INSERT INTO textcats \
            (text, is_ascii, length) \
            VALUES ($1, $2, $3)",
            &[
                &textcat.text, &textcat.is_ascii, &(textcat.length as i32),
            ],
        ).expect(&format!("Couldn't add cat '{:?}' to database!", textcat));
    }
}

