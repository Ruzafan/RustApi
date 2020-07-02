use crate::{common, User};
use mongodb::{bson::doc, bson::Document, error::Error, Collection};
use warp::{reply::Json, Rejection};

const USER_COLLECTION: &'static str = "Users";

pub async fn insert_user(name: String) -> Result<String, Rejection> {
    match insert(USER_COLLECTION, doc! { "Name": name}).await {
        Ok(names) => Ok(names),
        Err(_) => Err(warp::reject()),
    }
}

pub async fn get_user(name: String) -> Result<Json, Rejection> {
    match find_user(USER_COLLECTION, name).await {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(_) => Err(warp::reject()),
    }
}

async fn insert(collection_name: &str, document: mongodb::bson::Document) -> Result<String, Error> {
    let result: String = String::new();
    let collection = get_collection(collection_name).await?;
    collection.insert_one(document, None).await?;
    Ok(result)
}

async fn find_user(collection_name: &str, name: String) -> Result<Document, Error> {
    
    let collection = get_collection(collection_name).await?;
    let filter = doc! { "Name": name };

    let mongo_result = collection.find_one(filter, None).await?;

    Ok(mongo_result.unwrap())
}

async fn get_collection(collection: &str) -> Result<Collection, Error>
{
    let client = common::initialize_mongo().await?;
    let db = client.database("blog");
    Ok(db.collection(collection))
}