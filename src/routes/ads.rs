use mongodb::Database;
use rocket::{State, serde::json::Json};
use rocket_okapi::openapi;

use crate::{models::{ads::{AdInput, Status, AdsReportInput, PersonalInfo, AdGallery, PriceDetails}}, errors::response::MyError};


/// search and filter ads controller
#[openapi(tag = "Ads")]
#[get("/api/v1/user/search/ads")]
pub async fn search_ads_handler(
    db: &State<Database>,
) -> Result<Json<Vec<AdInput>>, MyError> {
    let st = Status{
        isApproved: true,
        isPublished: true,
        isFeatured: true,
        isActive: true,
    };
    let priceDetails =  PriceDetails{
        currency: String::from("default"),
        priceType: String::from("default"),
        price: 9999,
    };
    let personalInfo = PersonalInfo{
        fullName: String::from("default"),
        email: String::from("default"),
        phone: String::from("default"),
        location: String::from("default"),
    };

    let addrep = AdsReportInput {
        reason: String::from("default")
    };

    let adGalery = AdGallery{
            url: String::from("default"),
            public_id: String::from("default"),
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
    priceDetails: priceDetails,
    adGallery: vec!(adGalery),
    personalInfo: personalInfo,
    specialNote: String::from("default"),
    adsReport: vec!(addrep),
 
    };

    // let annoncement = AdInput{ data: String::from("bienvenue")};
    Ok(Json(vec!(annonce)))
}




