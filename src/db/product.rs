
use futures::TryStreamExt;
// use chrono::prelude::*;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::Database;
use mongodb::options::FindOptions;
use rocket::serde::json::Json;
use crate::models::product::{ProductInput, ProductDocument, Product};


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


pub async fn find_product(
    db: &Database,
    limit: i64,
    page: i64,
) -> mongodb::error::Result<Vec<Product>> {
    let collection = db.collection::<ProductDocument>("product");

    let find_options = FindOptions::builder()
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = collection.find(None, find_options).await?;

    let mut products: Vec<Product> = vec![];
    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let name = result.name;
        let created_at = result.createdAt;
        let price: u32 = result.price;
        // transform ObjectId to String
        let customer_json = Product {
            _id: _id.to_string(),
            name: name.to_string(),
            price: price,
            createdAt: created_at.to_string(),
        };
        products.push(customer_json);
    }

    Ok(products)
}