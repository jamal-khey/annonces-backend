
use super::rocket;
use rocket::tokio;
use std::env;
use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{Client};
use serde::{Deserialize, Serialize};


// #[test]
// fn hello_world() {
//     let client = Client::tracked(rocket()).expect("valid rocket instance");
//     let response = client.get("/").dispatch();
//     assert_eq!(response.status(), Status::Ok);
//     assert_eq!(
//         response.into_string().unwrap(),
//         serde_json::to_string(&MessageResponse {
//             message: "Hello World!".to_string()
//         })
//         .unwrap()
//     );
// }

// #[test]
// fn post_custumer() {
//     let client = Client::tracked(rocket()).expect("valid rocket instance");
//     let response = client.post("/custumer")
//         .header(ContentType::JSON)
//         .body(r##"{
//             "name": "John Doe",
//         }"##)
//         .dispatch();
//     assert_eq!(response.status(), Status::Ok);
//     // assert_eq!(
//     //     response.into_string().unwrap(),
//     //     serde_json::to_string(&MessageResponse {
//     //         message: "Hello World!".to_string()
//     //     })
//     //         .unwrap()
//     // );
// }

#[derive(Serialize, Deserialize)]
struct Student {
    name: String,
    grade: u32,
    test_scores: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::Collection;
    use tokio::runtime::Runtime;
    
    
    #[test]
    fn test_write_document() {
        dotenv().ok();
        let mongo_uri = env::var("MONGO_URI").expect("MONGO_DB_NAME is not found.");
        let mongo_db_name = env::var("MONGO_TEST_DB_NAME").expect("MONGO_DB_NAME is not found.");

        let rt = Runtime::new().unwrap();
        rt.block_on(async  {
            let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
            let client = Client::with_options(client_options).unwrap();
            let db = client.database(mongo_db_name.as_str());
            let collection = db.collection("User");


            let doc = doc! { "name": "John", "age": 30 };
            let _insert_one_result = collection.insert_one(doc.clone(), None).await.unwrap();
            print!("{}", doc.get_str("name").unwrap());
            

            let students: Collection<Student> = db.collection("students");

            let student = Student {
              name: "Emily".to_string(),
              grade: 10,
              test_scores: vec![98, 87, 100],
            };
            let result = students.insert_one(student, None).await;
            let student = students.find_one(doc! { "name": "Emily" }, None).await.unwrap();
            print!("{}", result.unwrap().inserted_id);
            match student {
                Some( data) => print!("{}", data.grade),
                None => todo!(),
            }
            

        });

    }
}
