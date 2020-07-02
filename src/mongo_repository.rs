use crate::{common, User};
use mongodb::{bson::{self, doc}, error::Error};
use warp::{Rejection, reply::Json};

const USER_COLLECTION: &'static str = "Users";

pub async fn insert_user(name: String) -> Result<String, Rejection> {
    match insert(USER_COLLECTION,doc! { "Name": name}).await {
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

async fn insert(collection: &str, document: mongodb::bson::Document) -> Result<String, Error>
{
    let result: String = String::new();
    let client = common::initialize_mongo().await?;
    let db = client.database("blog");
    let collection = db.collection(collection);

    collection.insert_one(document,None).await?;
    Ok(result)
}

async fn find_user(collection: &str, name: String) -> Result<User::UserData, Error>
{
    let client = common::initialize_mongo().await?;
    let db = client.database("blog");
    let collection = db.collection(collection);
    let filter = doc! { "Name": name };
    let document = collection.find_one(filter,None).await?;
    let mut user: User::UserData = document.unwrap();
    Ok(user)
}