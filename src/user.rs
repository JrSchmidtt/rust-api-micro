use rocket::serde::json::{json, Value};

#[get("/user")]
pub fn get_user() -> Value {
    json!({
        "name": "John Doe",
        "email": "user@server.com",
        "age": 42,
    })
}