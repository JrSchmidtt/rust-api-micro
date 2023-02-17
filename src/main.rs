#[macro_use] extern crate rocket;

// Import the `json` macro from the `rocket` crate.
use rocket::serde::json::{json, Value};

#[get("/")]
fn ping() -> Value {
    json!({
        "ping": "pong",
        "status": "ok",
        "message": "Hello, world!"
    })
}

#[get("/user")]
fn user() -> Value {
    json!({
        "name": "John Doe",
        "email": "user@server.com",
        "age": 42,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ping, user])
}