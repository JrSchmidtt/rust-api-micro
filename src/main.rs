#[macro_use]
extern crate rocket;

// Import the `json` macro from the `rocket` crate.
use rocket::serde::json::{json, Value};

// Import the `get_user` function from the `user` module
mod user;
use user::get_user;

#[get("/")]
fn ping() -> Value {
    json!({
        "ping": "pong",
        "status": "ok",
        "message": "Hello, world!"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ping, get_user])
}