
use mongodb::bson::doc;
use mongodb::Database;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi};


use crate::db::user;
use crate::models::response::MessageResponse;
use crate::models::user::{UserInput, CheckUserNameInput};


/// create a customer document
#[openapi(tag = "User")]
#[post("/api/v1/user/signup", data = "<input>")]
pub async fn signup(
    db: &State<Database>,
    input: Json<UserInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    // can set with a single error like this.
    match user::insert_user(&db, input).await {
        Ok(_user_doc_id) => {
            return Ok(Json(_user_doc_id));
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(BadRequest(Some(Json(MessageResponse {
                message: format!("Invalid input"),
            }))));
        }
    }
}



// /// Check user-name exist
#[openapi(tag = "User")]
#[get("/api/v1/user/check/username", data = "<input>")]
pub async fn check_user_name_exist(
    db: &State<Database>,
    input: Json<CheckUserNameInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    
    let user = user::check_user_name_exist(db, input.into_inner().userName).await;
    match user {
        Some(_) => Ok(Json("ok".to_owned())),
        None => Ok(Json("NotOk".to_owned())),
    }
    
}

// if (userNameExist) {
// 			return res.status(400).send({
// 				message: 'User Name already exist',
// 			})
// 		}
// 		res.status(200).send({
// 			message: 'User Name available',
// 		})