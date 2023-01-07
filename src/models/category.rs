
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CategoryStatus{
    pub isFeatured: bool,
    pub isActive: bool,
}


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryDocument {
    pub status: CategoryStatus,
    pub categoryTitle: String,
    pub slug: String,
    pub avatar: String,
    pub iconUrl: String,
}



#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Category {
    pub status: CategoryStatus,
    pub categoryTitle: String,
    pub slug: String,
    pub avatar: String,
    pub iconUrl: String,
}