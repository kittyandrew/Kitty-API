#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
// Third Party
use rocket_contrib::json::{Json, JsonValue};
use rocket::State;
// Own code
mod user;
use user::{ID, User, UserMap, generate_users};

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

// Home page

#[get("/")]
fn get_index() -> &'static str {
    "
    USAGE

      GET /users

          retrieves all available users (without pagination).

      GET /users/<id>

          retrieves the user with id `<id>` (if exists).
    "
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
	.mount("/users", routes![get_all_users, get_user_by_index])
        .register(catchers![not_found])
        .manage(generate_users())
	.launch();
}
