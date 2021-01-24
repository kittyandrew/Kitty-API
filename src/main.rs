#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
// Third Party
use rocket::tokio::time::{delay_for, Duration};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::http::uri::Origin;
use rocket::fairing::AdHoc;
use chrono::prelude::Utc;
use rocket::http::Method;
use blake3::hash;
// TODO: Look into using "Private Cookies" by Rocket
// use rocket::http::CookieJar;
// use rocket::request::Form;
// use either::Either;
use rocket::State;
// Standard
use std::collections::HashMap;
use std::env;
// Own code
mod entities;
use entities::{
    ID, User, UserMap, LoginMap, LoginCache, Data, Profile, Context,
    generate_users, new_session, get_login_storage, get_login_cache,
    get_context, reg_data_has_error, login_data_has_error
};

// Home page

#[get("/")]
fn get_index() -> Template {
    let hashmap = HashMap::<String, String>::new();
    Template::render("index", &hashmap)
}

// All API routes

// Users section

#[get("/")]
fn get_all_users(map: State<UserMap>) -> JsonValue {
    json!(map.lock().unwrap().values().collect::<Vec<&User>>())
}

#[delete("/")]
fn remove_all_users(map: State<UserMap>) -> JsonValue {
    map.lock().unwrap().clear();
    json!({ "message": "All users were removed!" })
}

#[get("/<id>")]
fn get_user_by_index(id: ID, map: State<UserMap>) -> JsonValue {
    match map.lock().unwrap().get(&id).map(|user| { json!(user) }) {
        Some(result) => result,
        None => json!({ "message": format!("User with ID {} does not exist!", id) }),
    }
}

#[delete("/<id>")]
fn remove_user_by_index(id: ID, map: State<UserMap>) -> JsonValue {
    match map.lock().unwrap().remove(&id) {
        Some(item) => json!({ "message": "Successfully removed user!", "user": item }),
        None => json!({ "message": format!("User with ID {} does not exist!", id) }),
    }
}

#[get("/?<page>")]
fn get_users_paginated(page: usize, map: State<UserMap>, context: State<Context>) -> JsonValue {
    let hashmap = map.lock().unwrap();
    let mut data = Vec::new();
    for n in 0..context.page_size {
        let id = context.page_size * page + n;
        hashmap.get(&id).map(|user| {
            data.push(user)
        });
    }
    json!({
        "page": page,
        "per_page": context.page_size,
        "items": data.len(),
        "next_exist": data.len() == context.page_size,
        "data": data,
    })
}

// Accounts section
// TODO: Rewrite this section

// Handling basic POST request with JSON data
#[post("/register", format = "application/json", data = "<data>")]
fn account_register(data: Json<Data>, login_map: State<LoginMap>, login_cache: State<LoginCache>) -> JsonValue {
    // Handle early returns
    if let Some(error) = reg_data_has_error(&data, &login_cache) {
        return error
    }

    // add to cache
    login_cache.lock().unwrap().insert(data.email.clone());
    // create Profile with (email, password hash, session)
    let pwd = hash(data.password.as_bytes()).to_hex().to_string();
    let session = new_session(24);
    // @Improve: decrease complexity by removing all .clone()
    login_map.lock().unwrap().insert(pwd.clone(), Profile {
        login: data.email.clone(),
        password: pwd.clone(),
        session: session.clone(),
    });

    json!({
        "message": "Registration success!",
        "session": session,
        "creation_date": Utc::now(),
    })
}

#[post("/login", data = "<data>")]
fn account_login(data: Json<Data>, login_map: State<LoginMap>, login_cache: State<LoginCache>) -> JsonValue {
    if let Some(error) = login_data_has_error(&data, &login_cache) {
        return error
    } else {
        // Recreate session on each login (meaning 1 session at a time)
        let session = new_session(24);
        // Unwrap mutable profiles map
        let mut profiles = login_map.lock().unwrap();
        // Get value or return early with error (password doesn't match)
        let profile = match profiles.get_mut(&hash(data.password.as_bytes()).to_hex().to_string()) {
            Some(val) => val,
            None => return json!({ "message": "Password was incorrect!" }),
        };
        profile.session = session.clone();

        json!({
            "message": "Login success!",
            "session": session,
            "creation_date": Utc::now(),
        })
    }
}

// Catch Errors

// This is AWFUL. MY GOD Rocket WHY
#[get("/")]
fn catch_not_auth() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Access denied! Authorization token is wrong or missing."
    })
}

// 404 page

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        // Home page
        .mount("/", routes![get_index])
        // API routes
        .mount("/api/users", routes![
            get_all_users, get_user_by_index, get_users_paginated,
            remove_all_users, remove_user_by_index,
        ])
        .mount("/api/accounts", routes![account_register, account_login])
        // Error route for TOKEN header handler
        .mount("/api/not_authorized", routes![catch_not_auth])
        // Serving static files (stylesheet, pictures etc)
        .mount("/static", StaticFiles::from("static"))
        // Attachements (Middleware)
        .attach(Template::fairing())
        .attach(AdHoc::on_request("Delay handler", |req, _| {
            Box::pin(async move {
                // path for delay must be /api, otherwise people will break something
                if !req.uri().path().starts_with("/api") { return }

                // unpack <delay>, skipping if at any point values are illegal or empty
                match req.get_query_value("delay") {
                    Some(val) => match val {
                        Ok(val) => match val {
                            Some(val) if val < 1 => return,
                            Some(val) if val > 10 => return,
                            Some(val) => delay_for(Duration::from_secs(val)).await,
                            None => return,
                        },
                        Err(_) => return,
                    },
                    None => return,
                };
            })
        }))
        // Catching the TOKEN auth header
        .attach(AdHoc::on_request("API Token handler", |req, _| {
            Box::pin(async move {
                // only /api path matters here
                if !req.uri().path().starts_with("/api") { return }

                // handle empty token -> we don't care if you supplied header then
                let token: String;
                match env::var("TOKEN") {
                    Ok(val) => token = val,
                    // early return
                    Err(_) => return,
                }

                let bad_uri = Origin::parse("/api/not_authorized").unwrap();
                match req.headers().get_one("Token") {
                    Some(val) => match val {
                        val if val.to_string() == token => return,
                        // This is AWFUL. MY GOD Rocket WHY
                        _val => {
                            req.set_uri(bad_uri);
                            req.set_method(Method::Get);
                        },
                    },
                    // This is AWFUL. MY GOD Rocket WHY
                    None => {
                        req.set_uri(bad_uri);
                        req.set_method(Method::Get);
                    },
                };
            })
        }))

        // All-catchers
        .register(catchers![not_found])
        // "local" vars
        .manage(generate_users())
        .manage(get_login_storage())
        .manage(get_login_cache())
        // Config
        .manage(get_context())
}
