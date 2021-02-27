#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate data_item_derive;
use rocket::tokio::time::{delay_for, Duration};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use data_item::{KittyBox, DataItem};
use rocket::http::uri::Origin;
use rocket::fairing::AdHoc;
use rocket::http::Method;
use std::env;


// Own code
mod preparation;
mod entities;
mod accounts;
mod headers;
mod utils;
mod misc;
// mod users;
// mod cats;


#[launch]
fn rocket() -> rocket::Rocket {
    // Generate data
    preparation::load_data();

    rocket::ignite()
        // Home page
        .mount("/", routes![misc::get_index])
        // API routes
        .mount("/api/users", entities::User::get_api_endpoints())
        .mount("/api/cats", entities::Cat::get_api_endpoints())
        .mount("/api/textcats", entities::TextCat::get_api_endpoints())
        .mount("/api/accounts", routes![
            accounts::account_register,
            accounts::account_login,
        ])
        // Error route for TOKEN header handler
        .mount("/api/not_authorized", routes![misc::catch_not_auth])
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
                        _ => {
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
        .register(catchers![misc::not_found])
        // Databases
        .attach(KittyBox::fairing())
        // "local" vars
        .manage(utils::get_login_storage())
        .manage(utils::get_login_cache())
        // Config
        .manage(utils::get_context())
}
