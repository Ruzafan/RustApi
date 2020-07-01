use crate::common;
use mongodb::error::Error;
use warp::Rejection;
use mongodb::bson::doc;

pub async fn insert_user(name: String) -> Result<String, Rejection> {
    match insert("Users",doc! { "Name": name}).await {
        Ok(names) => Ok(names),
        Err(_) => Err(warp::reject()),
    }
}

pub async fn insert(collection: &str, document: mongodb::bson::Document) -> Result<String, Error>
{
    let result: String = String::new();
    let client = common::initialize_mongo().await?;
    let db = client.database("blog");
    let collection = db.collection(collection);

    collection.insert_one(document,None).await?;
    Ok(result)
}