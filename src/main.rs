#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket::Request;
use rocket::http::Status;

mod db;
mod errors;
mod fairings;
mod models;
mod request_guards;
mod routes;

// #[catch(404)]
// fn not_found(req: &Request) -> String {
//     print!("{}",req.to_string());
//     format!("I couldn't find '{}'. Try something else?", req.uri())
// }

// #[catch(default)]
// fn default(status: Status, req: &Request) -> String {
//     format!("{} ({})", status, req.uri())
// }

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(db::init())
        .attach(fairings::cors::CORS)
        // .register("/", catchers![not_found, default])
        .mount(
            "/",
            openapi_get_routes![
                routes::index,
                routes::customer::get_customers,
                routes::customer::get_customer_by_id,
                routes::customer::post_customer,
                routes::customer::patch_customer_by_id,
                routes::customer::delete_customer_by_id,
                routes::product::get_products,
                routes::product::post_product,
                routes::admin::get_announcement,
                routes::ads::search_ads_handler,
                routes::category::get_category,
                routes::user::signup,
                routes::user::check_user_name_exist
            ],
        )
        .mount(
            "/api-docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}


// Unit testings
#[cfg(test)]
mod tests;
