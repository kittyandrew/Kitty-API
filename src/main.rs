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
    ID, User, UserMap, LoginMap, LoginCache, UserPage, Data, Profile,
    GoodRegResp, BadRegResp, AnyResp,
    generate_users, new_session, get_login_storage, get_login_cache,
    reg_data_has_error, login_data_has_error
};

// Config
static PAGINATION_SIZE: ID = 5;

// Home page

#[get("/")]
fn get_index() -> Template {
    let hashmap = HashMap::<String, String>::new();
    Template::render("index", &hashmap)
}

// All API routes

// Users section

#[get("/")]
fn get_all_users(map: State<UserMap>) -> AnyResp {
    let result = Json(map.lock().unwrap().values().cloned().collect());
    return AnyResp::GoodAll(result)
}

#[get("/<id>")]
fn get_user_by_index(id: ID, map: State<UserMap>) -> JsonValue {
    map.lock().unwrap().get(&id).map(|user| {
        json!(user)
    }).expect("Failed to create json!")
}

#[get("/?<page>")]
fn get_users_paginated(page: usize, map: State<UserMap>) -> JsonValue {
    let hashmap = map.lock().unwrap();
    let mut data = Vec::new();
    for n in 0..PAGINATION_SIZE {
        let id = PAGINATION_SIZE * page + n;
        hashmap.get(&id).map(|user| {
            data.push(user.clone())
        });
    }
    json!(UserPage {
        page: page,
        per_page: PAGINATION_SIZE,
        items: data.len(),
        next_exist: data.len() == PAGINATION_SIZE,
        data: data,
    })
}

// Register - Login - Get page with session

// Handling basic POST request with JSON data
#[post("/register", format = "application/json", data = "<data>")]
fn account_register(data: Json<Data>, login_map: State<LoginMap>, login_cache: State<LoginCache>) -> AnyResp {
    // Handle early returns
    if let Some(error) = reg_data_has_error(&data, &login_cache) {
        return error
    } else {
        let mut cache = login_cache.lock().unwrap();
        // add to cache
        cache.insert(data.email.clone());
        // create Profile with (email, password hash, session)
        let mut logins = login_map.lock().unwrap();
        let pwd = hash(data.password.as_bytes()).to_hex().to_string();
        let session = new_session(24);
        logins.insert(pwd.clone(), Profile {
            login: data.email.clone(),
            password: pwd.clone(),
            session: session.clone(),
        });
        return AnyResp::Good(Json(GoodRegResp {
            message: "Registration success!".to_string(),
            session: session,
            creation_date: Utc::now(),
        }))
    }
}

#[post("/login", data = "<data>")]
fn account_login(data: Json<Data>, login_map: State<LoginMap>, login_cache: State<LoginCache>) -> AnyResp {
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
            None => return AnyResp::Bad(Json(BadRegResp { message: "Password was incorrect!".to_string() })),
        };
        profile.session = session.clone();
        return AnyResp::Good(Json(GoodRegResp {
            message: "Login success!".to_string(),
            session: session,
            creation_date: Utc::now(),
        }))
    }
}

// Catch Errors

// This is AWFUL. MY GOD rocket WHY
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
        .mount("/api/users", routes![get_all_users, get_user_by_index, get_users_paginated])
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
        // This is AWFUL. MY GOD rocket WHY
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
                        _val => {
                            req.set_uri(bad_uri);
                            req.set_method(Method::Get);
                        },
                    },
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
}
