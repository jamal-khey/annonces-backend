use mongodb::bson::doc;
use mongodb::Database;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;

use crate::models::response::MessageResponse;

use crate::db::product;
use crate::models::product::ProductInput;

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
