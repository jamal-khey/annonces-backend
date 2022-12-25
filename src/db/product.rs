
// use chrono::prelude::*;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::Database;
use rocket::serde::json::Json;
use crate::models::product::ProductInput;


pub async fn insert_product(
    db: &Database,
    input: Json<ProductInput>,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<Document>("product");

    let created_at: DateTime = DateTime::now();

    let insert_one_result = collection
        .insert_one(
            doc! { "name": input.name.clone(),
                        "price": input.price,
                        "createdAt": created_at},
            None,
        )
        .await?;

    Ok(insert_one_result.inserted_id.to_string())
}