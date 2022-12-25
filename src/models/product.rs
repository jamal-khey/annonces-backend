use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductDocument {
    /// Document Id
    pub _id: ObjectId,
    /// customer name
    pub name: String,
    pub price: u32,
    /// createdAt
    pub createdAt: DateTime,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Product {
    /// Document Id
    pub _id: String,
    /// product name
    pub name: String,
    //product price
    pub price: u32,
    /// createdAt
    pub createdAt: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct ProductInput {
    /// customer name
    pub name: String,
    pub price: u32,
}