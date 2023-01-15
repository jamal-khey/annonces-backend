
use mongodb::bson::doc;
use mongodb::Database;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;


use crate::db::user;
use crate::models::response::MessageResponse;
use crate::models::user::UserInput;


/// create a customer document
#[openapi(tag = "User")]
#[post("/user/signup", data = "<input>")]
pub async fn signup(
    db: &State<Database>,
    input: Json<UserInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    // can set with a single error like this.
    match user::insert_user(&db, input).await {
        Ok(_product_doc_id) => {
            return Ok(Json(_product_doc_id));
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(BadRequest(Some(Json(MessageResponse {
                message: format!("Invalid input"),
            }))));
        }
    }
}


