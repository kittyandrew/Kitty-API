use rocket_contrib::json::{Json, JsonValue};
use rocket::State;
// Own code
use crate::entities::{ID, User, KittyBox};
use crate::headers::PageSize;


#[get("/")]
pub async fn get_all_users(conn: KittyBox) -> JsonValue {
    json!({
        "msg_code": "no_message",
        "data": conn.run(
            |c| c.query("SELECT * FROM users", &[])
            .unwrap()
            .iter()
            .map(|row| User::from_row(row))
            .collect::<Vec<User>>()
        ).await,
    })
}


#[delete("/")]
pub async fn remove_all_users(conn: KittyBox) -> JsonValue {
    conn.run(
        |c| {
            let count: i64 = c.query_one("SELECT count(*) FROM users", &[])
                .unwrap()
                .get("count");
            // @UseCase: do we want reset identity here? Probably yes.
            c.execute("TRUNCATE TABLE users RESTART IDENTITY", &[])
                .expect("Fatal error when cleaning users table!");
            json!({
                "msg_code": "info_users_removed",
                // "message": context.format_usize("info_users_removed", &vec![size])
                "users_removed": count,
            })
        }
    ).await
}


#[post("/", format = "application/json", data = "<user>")]
pub async fn create_new_user(user: Json<User>, conn: KittyBox) -> JsonValue {
    conn.run(
        |c| json!({
            "msg_code": "info_new_user_ok",
            // "message": context.get_message("info_new_user_ok"),
            "user_id": user.into_inner().insert(c),
        })
    ).await
}


// User per <id> section


#[get("/<id>")]
pub async fn get_user_by_index(id: ID, conn: KittyBox) -> JsonValue {
    let row = conn.run(
        move |c| c.query_one("SELECT * FROM users WHERE id = $1", &[&(id as i32)])
    ).await;

    match row {
        Ok(r) => json!({
            "msg_code": "no_info",
            "user_id": id,
            "data": User::from_row(&r),
        }),
        Err(_) => json!({
            "msg_code": "err_user_not_exist",
            "user_id": id,
            // "message": ,
        }),
    }
}


#[delete("/<id>")]
pub async fn remove_user_by_index(id: ID, conn: KittyBox) -> JsonValue {
    let row = conn.run(
        move |c| c.query_one("DELETE FROM users WHERE id = $1 RETURNING *", &[&(id as i32)])
    ).await;

    match row {
        Ok(r) => json!({
            "msg_code": "info_remove_user_ok",
            // "message": ,
            "data": User::from_row(&r),
        }),
        Err(_) => json!({
            "msg_code": "err_user_not_exist",
            "user_id": id,
            // "message": ,
        }),
    }
}


#[post("/<id>", format = "application/json", data = "<user>")]
pub async fn create_new_user_by_index(id: ID, user: Json<User>, conn: KittyBox) -> JsonValue {
    conn.run(
        move |c| {
            let mut db_user = user.into_inner();
            db_user.id = id;

            if let Ok(_) = db_user.insert_with_id(c) {
                json!({
                    "msg_code": "info_new_user_ok",
                    // "message": ,
                    "user_id": &db_user.id,
                })
            } else {
                json!({
                    "msg_code": "err_user_exists",
                    // "message": ,
                    "user_id": &db_user.id,
                })
            }
        }
    ).await
}


#[put("/<id>", format = "application/json", data = "<user>")]
pub async fn put_user_by_index(id: ID, user: Json<User>, conn: KittyBox) -> JsonValue {
    conn.run(
        move |c| {
            let mut db_user = user.into_inner();
            db_user.id = id;
            json!({
                "msg_code": "info_user_put_ok",
                // "message": context.get_message("info_new_user_ok"),
                "user_id": db_user.put(c),
            })
        }
    ).await
}


// Handler for pagination


#[get("/?<page>")]
pub async fn get_users_paginated(page: usize, page_size: PageSize, conn: KittyBox) -> JsonValue {
    json!({
        "msg_code": "no_message",
        "page_number": page.clone(),
        "page_size": page_size.0.clone(),
        "data": conn.run(
            move |c| c.query(
                    "SELECT * FROM users ORDER BY id ASC LIMIT $1 OFFSET $2",
                    &[&(page_size.0 as i64), &((page * page_size.0) as i64)]
                )
                .unwrap()
                .iter()
                .map(|row| User::from_row(row))
                .collect::<Vec<User>>()
        ).await,
    })
}
