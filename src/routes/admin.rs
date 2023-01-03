use mongodb::Database;
use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;

use crate::{models::admin::Announcement, errors::response::MyError};


/// get customer document by _id
#[openapi(tag = "Admin")]
#[get("/api/v1/admin/announcement")]
pub async fn get_announcement(
    db: &State<Database>,
) -> Result<Json<Announcement>, MyError> {

    let annoncement = Announcement{ data: String::from("bienvenue")};
    Ok(Json(annoncement))
}
