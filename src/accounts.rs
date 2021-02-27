use rocket_contrib::json::{Json, JsonValue};
use chrono::prelude::Utc;
use rocket::State;
use blake3::hash;
// Own code
use crate::entities::{
    LoginMap, LoginCache, Data, Profile, Context,
};
use crate::utils::{
    new_session, reg_data_has_error, login_data_has_error
};


// Accounts section
// TODO: Rewrite this section


// Handling basic POST request with JSON data
#[post("/register", format = "application/json", data = "<data>")]
pub fn account_register(data: Json<Data>, login_map: State<LoginMap>, login_cache: State<LoginCache>, context: State<Context>) -> JsonValue {
    // Handle early returns
    if let Some(error) = reg_data_has_error(&data, &login_cache, &context) {
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
        "msg_code": "info_reg_ok",
        "message": context.get_message("info_reg_ok"),
        "data": {
            "session": session,
            "creation_date": Utc::now(),
        }
    })
}


#[post("/login", data = "<data>")]
pub fn account_login(data: Json<Data>, login_map: State<LoginMap>, login_cache: State<LoginCache>, context: State<Context>) -> JsonValue {
    if let Some(error) = login_data_has_error(&data, &login_cache, &context) {
        return error
    } else {
        // Recreate session on each login (meaning 1 session at a time)
        let session = new_session(24);
        // Unwrap mutable profiles map
        let mut profiles = login_map.lock().unwrap();
        // Get value or return early with error (password doesn't match)
        let profile = match profiles.get_mut(&hash(data.password.as_bytes()).to_hex().to_string()) {
            Some(val) => val,
            None => return json!({
                "msg_code": "err_bad_credentials",
                "message": context.get_message("err_bad_credentials")
            }),
        };
        profile.session = session.clone();

        json!({
            "msg_code": "info_login_ok",
            "message": context.get_message("info_login_ok"),
            "data": {
                "session": session,
                "creation_date": Utc::now(),
            }
        })
    }
}

