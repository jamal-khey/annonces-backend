use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct FullName{
  pub firstName: String,
  pub lastName: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct PackageDetails{
  // pub packageId: PackageDocument['_id'], //TODO to be implemented
  pub packageName: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct RatingDetails{
  pub averageRating: u32,
  pub reviewCount: u32
}


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Role{
  pub isBuyer: bool,
  pub isSeller: bool,
  pub isAdmin: bool,
  pub isSuperAdmin: bool,
}
    

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDocument {
  pub _id: ObjectId,
  pub fullName: FullName,
  pub userName: String,
  pub email: String,
  pub password: String,
  pub isConfirmed: bool,
  pub packageDetails: PackageDetails,
  pub ratingDetails: RatingDetails,
  //pub sellerReviews: Vec<SellerReviews>, //TODO enable seller review
  pub resetLink: String,
  pub phoneNumber: String,
  pub avatar: String,
  pub cloudinary_id: String,
  pub role: Role,
  pub createdAt: DateTime,
  pub updatedAt: DateTime
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
  pub _id: String,
  pub fullName: FullName,
  pub userName: String,
  pub email: String,
  pub password: String,
  pub isConfirmed: bool,
  pub packageDetails: PackageDetails,
  pub ratingDetails: RatingDetails,
  pub resetLink: String,
  pub phoneNumber: String,
  pub avatar: String,
  pub cloudinary_id: String,
  pub role: Role,
  pub createdAt: DateTime,
  pub updatedAt: DateTime
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct UserInput {
    pub fullName: FullName,
    pub userName: String,
    pub email: String,
    pub password: String,
    pub isConfirmed: bool,
    pub packageDetails: Option<PackageDetails>,
    pub ratingDetails: Option<RatingDetails>,
    pub resetLink: Option<String>,
    pub phoneNumber: Option<String>,
    pub avatar: Option<String>,
    pub cloudinary_id: Option<String>,
    pub role: Role
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CheckUserNameInput{
    pub userName: String
}