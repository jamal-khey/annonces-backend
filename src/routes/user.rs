
use mongodb::bson::doc;
use mongodb::Database;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi};


use crate::db::user::{self, find_user};
use crate::errors::response::MyError;
use crate::models::response::MessageResponse;
use crate::models::user::{UserInput, CheckUserNameInput, LoginInput};


/// create a customer document
#[openapi(tag = "User")]
#[post("/api/v1/user/signup", data = "<input>")]
pub async fn signup(
    db: &State<Database>,
    input: Json<UserInput>,
) -> Result<Json<MessageResponse>, BadRequest<Json<MessageResponse>>> {
    // can set with a single error like this.
    match user::insert_user(&db, input).await {
        Ok(_user_doc_id) => {
            return Ok(Json(MessageResponse {
                message: format!("User added"),
            }));
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
) -> Result<Json<MessageResponse>, BadRequest<Json<MessageResponse>>> {
    
    let user = user::check_user_name_exist(db, input.into_inner().userName).await;
    match user {
        None => {
            Ok(Json(MessageResponse { message: format!("User Name available"),}
        ))
        },
        Some(_) => {
            return Err(BadRequest(Some(Json(MessageResponse {
                message: format!("User Name already exist"),
            }))));
        }
    }
}


#[openapi(tag = "User")]
#[post("/api/v1/user/session", data = "<input>")]
pub async fn create_user_session(
    db: &State<Database>,
    input: Json<LoginInput>,
) -> Result<Json<MessageResponse>, MyError> {
    let deserialized_user = input.into_inner();
    
    // Validate the user's password if Err -> 401
    
    //create session
    //  Ok -> 200 { message: 'User Logged In', accessToken: accessToken, refreshToken: refreshToken}
    
    match user::find_user(db, deserialized_user.email).await {
        Ok(user) => {
            match user {
                Some( _user_payload) => Ok(Json(MessageResponse { message: format!("User Name available"),})),
                None => Err(MyError::build(401, Some("user not found".to_string())))
            }
            
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(401, Some("user not found".to_string())));
        }
    }
   
}