// use mongodb::bson::{doc, Document};

use mongodb::{Database, Collection, bson::doc};
use crate::models::user::{UserInput};
// use chrono::prelude::*;
use mongodb::bson::{DateTime};
use rocket::serde::json::Json;


pub async fn insert_user(
    db: &Database,
    input: Json<UserInput>,
) -> mongodb::error::Result<String> {
    
    let collection = db.collection::<UserInput>("User");

    let _created_at: DateTime = DateTime::now();
    let user_data = input.into_inner();
    let insert_one_result = collection
        .insert_one(
            user_data,
            None,
        )
        .await?;

    Ok(insert_one_result.inserted_id.to_string())
}

pub async fn check_user_name_exist(
    db: &Database,
    user_name: String,
) -> Option<UserInput> {

    let user_collection: Collection<UserInput> = db.collection("User");
    let result = user_collection.find_one(doc! { "userName": user_name }, None).await;
    let none: Option<UserInput> = None;
    match result {
        Ok(user) => return user,
        Err(e) => {
            print!("{}",e);
            return none
        }
    };

}


pub async fn find_user(
  db: &Database,
  email: String) -> mongodb::error::Result<Option<UserInput>> {

    let collection = db.collection::<UserInput>("User");
    let user_doc= collection.find_one(doc! { "email": email }, None).await?;
    if user_doc.is_none() {
      print!("{}", "is_none******");
      return Ok(None);
    }
    let unwrapped_doc = user_doc.unwrap();


    Ok(Some(unwrapped_doc))
            
  }