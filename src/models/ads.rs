use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdDocument {
    pub _id: ObjectId,
    pub status: Status,
    pub adTitle: String,
    pub slug: String,
    pub category: String,
    pub location: String,
    pub tags: Vec<String>,
    pub itemCondition: String,
    pub itemWarranty: String,
    pub adType: String,
    pub description: String,
    pub specifications: Vec<String>,
    // priceDetails: {
    //     currency: String,
    //     priceType: String,
    //     price: number
    // }
    // adGallery: [
    //     {
    //         url: String,
    //         public_id: String,
    //     }
    // ]
    // personalInfo: {
    //     fullName: String,
    //     email: String,
    //     phone: String,
    //     location: String,
    // }
    pub specialNote: String,
    // adsReport: [
    //     {
    //         user: UserDocument['_id']
    //         reason: String,
    //     }
    // ]
    pub createdAt: DateTime,
    pub updatedAt: DateTime,
    pub expireAt: DateTime,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Status{
    pub isApproved: bool,
    pub isPublished: bool,
    pub isFeatured: bool,
    pub isActive: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct PriceDetails {
    pub currency: String,
    pub priceType: String,
    pub price: u64
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct AdGallery{
    pub url: String,
    pub public_id: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct PersonalInfo {
    pub fullName: String,
    pub email: String,
    pub phone: String,
    pub location: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdsReport {
    pub _id: ObjectId,
    pub reason: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct AdsReportInput {
    pub reason: String,
}


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ad {
    pub _id: ObjectId,
    pub status: Status,
    pub adTitle: String,
    pub slug: String,
    pub category: String,
    pub location: String,
    pub tags: Vec<String>,
    pub itemCondition: String,
    pub itemWarranty: String,
    pub adType: String,
    pub description: String,
    pub specifications: Vec<String>,
    // priceDetails: {
    //     currency: String,
    //     priceType: String,
    //     price: number
    // }
    // adGallery: [
    //     {
    //         url: String,
    //         public_id: String,
    //     }
    // ]
    // personalInfo: {
    //     fullName: String,
    //     email: String,
    //     phone: String,
    //     location: String,
    // }
    pub specialNote: String,
    // adsReport: [
    //     {
    //         user: UserDocument['_id']
    //         reason: String,
    //     }
    // ]
    pub createdAt: DateTime,
    pub updatedAt: DateTime,
    pub expireAt: DateTime,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct AdInput {

    pub status: Status,
    pub adTitle: String,
    pub slug: String,
    pub category: String,
    pub location: String,
    pub tags: Vec<String>,
    pub itemCondition: String,
    pub itemWarranty: String,
    pub adType: String,
    pub description: String,
    pub specifications: Vec<String>,
    pub priceDetails: PriceDetails,
    pub adGallery: Vec<AdGallery>,
    pub personalInfo: PersonalInfo,
    pub specialNote: String,
    pub adsReport: Vec<AdsReportInput>,
}