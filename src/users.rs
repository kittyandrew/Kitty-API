// Third Party
use rocket_contrib::json::{Json, JsonValue};
use rocket::State;
// Own code
use crate::entities::{
    ID, User, UserMap, Context,
};
use crate::utils::{
    get_free_index
};

// Users section root

#[get("/")]
pub fn get_all_users(map: State<UserMap>) -> JsonValue {
    json!({
        "msg_code": "no_info",
        "data": map.lock().unwrap().values().collect::<Vec<&User>>()
    })
}

#[delete("/")]
pub fn remove_all_users(map: State<UserMap>, context: State<Context>) -> JsonValue {
    let mut hashmap = map.lock().unwrap();
    let size = hashmap.len();
    hashmap.clear();
    json!({
        "msg_code": "info_users_removed",
        "users_removed": size,
        "message": context.format_usize("info_users_removed", &vec![size])
    })
}

#[post("/", format = "application/json", data = "<user>")]
pub fn create_new_user(user: Json<User>, map: State<UserMap>, context: State<Context>) -> JsonValue {
    let mut hashmap = map.lock().unwrap();
    let mut user = user.into_inner();
    // Do not look for free space if empty, just add first
    if hashmap.is_empty() {
        user.id = 0;
        hashmap.insert(0, user.clone());
        return json!({
            "msg_code": "info_new_user_ok",
            "message": context.get_message("info_new_user_ok"),
            "data": user
        })
    };
    let index: usize = get_free_index(&hashmap);
    // Filling empty slot with new user
    user.id = index;
    hashmap.insert(index, user.clone());
    json!({
        "msg_code": "info_new_user_ok",
        "message": context.get_message("info_new_user_ok"),
        "data": user
    })
}

// User per <id> section

#[get("/<id>")]
pub fn get_user_by_index(id: ID, map: State<UserMap>, context: State<Context>) -> JsonValue {
    match map.lock().unwrap().get(&id).map(|user| { user }) {
        Some(user) => json!({
            "msg_code": "no_info",
            "id": id,
            "data": user,
        }),
        None => json!({
            "msg_code": "err_user_not_exist",
            "id": id,
            "message": context.format_usize("err_user_not_exist", &vec![id])
        }),
    }
}

#[delete("/<id>")]
pub fn remove_user_by_index(id: ID, map: State<UserMap>, context: State<Context>) -> JsonValue {
    match map.lock().unwrap().remove(&id) {
        Some(user) => json!({
            "msg_code": "info_remove_user_ok",
            "message": context.get_message("info_remove_user_ok"),
            "data": user
        }),
        None => json!({
            "msg_code": "err_user_not_exist",
            "id": id,
            "message": context.format_usize("err_user_not_exist", &vec![id])
        }),
    }
}

#[post("/<id>", format = "application/json", data = "<user>")]
pub fn create_new_user_by_index(id: ID, user: Json<User>, map: State<UserMap>, context: State<Context>) -> JsonValue {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        return json!({
            "msg_code": "err_user_exists",
            "id": id,
            "message": context.format_usize("err_user_exists", &vec![id])
        })
    }

    let mut user = user.into_inner();
    user.id = id;
    hashmap.insert(id, user.clone());
    return json!({
        "msg_code": "info_new_user_ok",
        "message": context.get_message("info_new_user_ok"),
        "data": user
    })
}

#[put("/<id>", format = "application/json", data = "<user>")]
pub fn put_user_by_index(id: ID, user: Json<User>, map: State<UserMap>, context: State<Context>) -> JsonValue {
    let mut hashmap = map.lock().unwrap();
    let mut user = user.into_inner();
    user.id = id;
    hashmap.insert(id, user.clone());
    return json!({
        "msg_code": "info_user_put_ok",
        "message": context.format_usize("info_user_put_ok", &vec![id]),
        "data": user
    })
}

// Handler for pagination

#[get("/?<page>")]
pub fn get_users_paginated(page: usize, map: State<UserMap>, context: State<Context>) -> JsonValue {
    let hashmap = map.lock().unwrap();
    let mut data = Vec::new();
    for n in 0..context.page_size {
        let id = context.page_size * page + n;
        hashmap.get(&id).map(|user| {
            data.push(user)
        });
    }
    json!({
        "msg_code": "no_info",
        "data": {
            "page_number": page,
            "page_size": context.page_size,
            "page_length": data.len(),
            "has_next": data.len() == context.page_size,
            "page": data,
        }
    })
}

