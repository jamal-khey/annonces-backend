use super::rocket;
use crate::models::response::MessageResponse;
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use serde_json;

#[test]
fn hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().unwrap(),
        serde_json::to_string(&MessageResponse {
            message: "Hello World!".to_string()
        })
        .unwrap()
    );
}

#[test]
fn post_custumer() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.post("/custumer")
        .header(ContentType::JSON)
        .body(r##"{
            "name": "John Doe",
        }"##)
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    // assert_eq!(
    //     response.into_string().unwrap(),
    //     serde_json::to_string(&MessageResponse {
    //         message: "Hello World!".to_string()
    //     })
    //         .unwrap()
    // );
}