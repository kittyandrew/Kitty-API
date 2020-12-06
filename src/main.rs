#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
// Third Party
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::State;
// Standard
use std::collections::HashMap;
// Own code
mod user;
use user::{ID, User, UserMap, UserPage, generate_users};

// Config
static PAGINATION_SIZE: ID = 5;

// All API routes

// Users section

#[get("/")]
fn get_all_users(map: State<UserMap>) -> Json<Vec<User>> {
    let hashmap = map.lock().unwrap();
    Json(hashmap.values().cloned().collect())
}

#[get("/<id>")]
fn get_user_by_index(id: ID, map: State<UserMap>) -> Option<Json<User>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|user| {
	Json(user.clone())
    })
}

#[get("/?<page>")]
fn get_users_paginated(page: usize, map: State<UserMap>) -> Json<UserPage> {
    let hashmap = map.lock().unwrap();
    let mut data = Vec::new();
    for n in 1..PAGINATION_SIZE + 1 {
	let id = PAGINATION_SIZE * page + n;
	hashmap.get(&id).map(|user| {
	    data.push(user.clone())
	});
    }
    Json(UserPage {
        page: page,
        page_size: PAGINATION_SIZE,
        returned_size: data.len(),
	next_exist: data.len() < PAGINATION_SIZE,
	data: data,
    })
}

// Home page

#[get("/")]
fn get_index() -> Template {
    let hashmap = HashMap::<String, String>::new();
    Template::render("index", &hashmap)
}

// 404 page

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn main() {
    rocket::ignite()
	.mount("/", routes![get_index])
	.mount("/api/users", routes![get_all_users, get_user_by_index, get_users_paginated])
	.mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .register(catchers![not_found])
        .manage(generate_users())
	.launch();
}
