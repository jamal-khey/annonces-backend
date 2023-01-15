// use mongodb::bson::{doc, Document};

use mongodb::{Database};
use crate::models::user::{UserInput};
// use chrono::prelude::*;
use mongodb::bson::{doc, DateTime, Document};
use rocket::serde::json::Json;


pub async fn insert_user(
    db: &Database,
    input: Json<UserInput>,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<Document>("User");

    let created_at: DateTime = DateTime::now();

    let insert_one_result = collection
        .insert_one(
            doc! {"name": input.userName.clone(), "createdAt": created_at},
            None,
        )
        .await?;

    Ok(insert_one_result.inserted_id.to_string())
}
