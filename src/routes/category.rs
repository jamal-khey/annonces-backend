use mongodb::Database;
use rocket::{State, serde::json::Json};
use rocket_okapi::{openapi, JsonSchema};
use serde::{Deserialize, Serialize};

use crate::{models::category::{Category, CategoryStatus}, errors::response::MyError};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct AllCategory{
    message: String,
    data: Vec<Category>
}

/// get customer document by _id
#[openapi(tag = "Category")]
#[get("/api/v1/ads/category")]
pub async fn get_category(
    _db: &State<Database>,
) -> Result<Json<AllCategory>, MyError> {

    let status_category = CategoryStatus{
        isFeatured: true,
        isActive: true,
    };

    let category = Category{ 
        status: status_category,
        categoryTitle: String::from("electronique"),
        slug: String::from("slug"),
        avatar: String::from("avatar"),
        iconUrl: String::from("favicon.ico"),
    };

    let categories = AllCategory{
        message: String::from("Successfully find categories"),
        data: vec!(category)
    };
    Ok(Json(categories))
}
