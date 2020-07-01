use crate::common;
use mongodb::error::Error;
use warp::Rejection;
use mongodb::bson::doc;

pub async fn init_mongo_and_insert() -> Result<String, Rejection> {
    match insert_user().await {
        Ok(names) => Ok(names),
        Err(_) => Err(warp::reject()),
    }
}

pub async fn insert_user() -> Result<String, Error>
{
    let result: String = String::new();
    let client = common::initialize_mongo().await?;
    let db = client.database("blog");
    let collection = db.collection("Users");

    let user = doc! { "Name":"Marc"};

    collection.insert_one(user,None).await?;
    Ok(result)
}