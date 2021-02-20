use crate::utils::generate_users;
use postgres::{Client, NoTls};


// const DB_URL: &str = "postgres://kitty:hackme@kitty-api-db:5432/kittybox";
const DB_URL: &str = "postgres://kitty:hackme@localhost:5432/kittybox";


// A bit ugly and very manual, but this works for now
pub fn generate_data() {
    let mut client = Client::connect(&DB_URL, NoTls)
        .expect("Couldn't connect to db for preparation (loading data)!");
    // First wipe out the table, and restart ID count
    client.execute("TRUNCATE TABLE users RESTART IDENTITY", &[]).unwrap();

    for user in generate_users() {
        client.execute(
            "INSERT INTO users \
             (username, first_name, last_name, email, age, active, picture) \
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
            &[
                &user.username, &user.first_name, &user.last_name,
                &user.email, &(user.age as i32), &user.active, &user.picture,
            ],
        ).expect(&format!("Couldn't add user '{:?}' to database!", user));
    }
}
