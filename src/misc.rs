use rocket_contrib::templates::Template;
use rocket_contrib::json::JsonValue;
use rocket::State;
// Own code
use crate::entities::Context;


// Home page


#[get("/")]
pub fn get_index(context: State<Context>) -> Template {
    Template::render("index", &context.inner())
}


// Catch Errors


// This is AWFUL. MY GOD Rocket WHY
#[get("/")]
pub fn catch_not_auth(context: State<Context>) -> JsonValue {
    json!({
        "msg_code": "err_access_denied",
        "message": context.get_message("err_access_denied")
    })
}


// 404 page


#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "msg_code": "err_res_not_found",
        // TODO: broken. "message": context.get_message("err_res_not_found")
    })
}

