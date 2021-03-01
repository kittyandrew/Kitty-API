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
#[get("/")]
pub fn catch_not_auth() -> JsonValue {
    json!({
        "msg_code": "err_access_denied",
        "message": "Access denied! Authorization token is wrong or missing.",
    })
}


// 404 page
#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "msg_code": "err_res_not_found",
        "message": "Resource not found! Make you sure your request path and data are correct.",
    })
}

