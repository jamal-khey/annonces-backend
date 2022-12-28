use rocket::serde::json::Json;
use rocket_okapi::openapi;
use rocket::serde::json::{json, Value};


pub mod customer;
pub mod product;


/// This is a description. <br />You can do simple html <br /> like <b>this<b/>
#[openapi(tag = "root")]
#[get("/")]
pub fn index() -> Value {
    json!({
        "key": "value",
        "array": [1, 2, 3, 4]
    })
}
