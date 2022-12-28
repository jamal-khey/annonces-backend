use mongodb::bson::doc;
use mongodb::Database;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;

use crate::errors::response::MyError;
use crate::models::response::MessageResponse;

use crate::db::product;
use crate::models::product::{ProductInput, Product};

/// create a customer document
#[openapi(tag = "Product")]
#[post("/product", data = "<input>")]
pub async fn post_product(
    db: &State<Database>,
    input: Json<ProductInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    // can set with a single error like this.
    match product::insert_product(&db, input).await {
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

/// get customer documents
#[openapi(tag = "Product")]
#[get("/product?<limit>&<page>")]
pub async fn get_products(
    db: &State<Database>,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<Json<Vec<Product>>, MyError> {
    // Error handling
    // This is also valid when strict checking is necessary.
    // if limit < 0 {
    //     return Err(BadRequest(Some(Json(MessageResponse {
    //         message: "limit cannot be less than 0".to_string(),
    //     }))));
    // }
    // if !page.is_none() && page.unwrap() < 1 {
    //     return Err(BadRequest(Some(Json(MessageResponse {
    //         message: "page cannot be less than 1".to_string(),
    //     }))));
    // }

    // Setting default values
    let limit: i64 = limit.unwrap_or(12);
    let page: i64 = page.unwrap_or(1);
    match product::find_product(&db, limit, page).await {
        Ok(_product_docs) => Ok(Json(_product_docs)),
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(400, Some(_error.to_string())));
        }
    }
}
