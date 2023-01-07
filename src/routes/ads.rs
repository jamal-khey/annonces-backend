use mongodb::Database;
use rocket::{State, serde::json::Json};
use rocket_okapi::{openapi, JsonSchema};

use crate::{models::{ads::{AdInput, Status, AdsReportInput, PersonalInfo, AdGallery, PriceDetails}}, errors::response::MyError};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct AllAds {
    message: String,
    data: Vec<AdInput>
}
/// search and filter ads controller
#[openapi(tag = "Ads")]
#[get("/api/v1/user/search/ads")]
pub async fn search_ads_handler(
    db: &State<Database>,
) -> Result<Json<AllAds>, MyError> {
    let st = Status{
        isApproved: true,
        isPublished: true,
        isFeatured: true,
        isActive: true,
    };
    let price_details =  PriceDetails{
        currency: String::from("$"),
        priceType: String::from("default"),
        price: 9999,
    };
    let personal_info = PersonalInfo{
        fullName: String::from("default"),
        email: String::from("default"),
        phone: String::from("default"),
        location: String::from("default"),
    };

    let addrep = AdsReportInput {
        reason: String::from("default")
    };

    let ad_galery = AdGallery{
            url: String::from("https://m.media-amazon.com/images/I/71DNwYciRsL._SL1500_.jpg"),
            public_id: String::from(""),
    };
    
    
    let annonce = AdInput {
    status: st,
    adTitle: String::from("default"),
    slug: String::from("default"),
    category: String::from("default"),
    location: String::from("default"),
    tags: vec!(String::from("default")),
    itemCondition: String::from("default"),
    itemWarranty: String::from("default"),
    adType: String::from("default"),
    description: String::from("default"),
    specifications: vec!(String::from("default")),
    priceDetails: price_details,
    adGallery: vec!(ad_galery),
    personalInfo: personal_info,
    specialNote: String::from("default"),
    adsReport: vec!(addrep),
 
    };

    let add_ads = AllAds{
        message: String::from("Successfully find all ads"),
        data: vec!(annonce)
    };
    Ok(Json(add_ads))
}




