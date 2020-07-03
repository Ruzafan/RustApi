use crate::{common,user};
use mongodb::{bson::{self, doc}, bson::Document, error::Error, Collection};
use warp::{reply::Json, Rejection};

const USER_COLLECTION: &'static str = "Users";

pub async fn insert_user(user: user::user_data) -> Result<String, Rejection> {
    let user_doc:Document = doc! {
        "Name": user.name,
        "Age": user.age,
        "Email": user.email
    };
    match insert(USER_COLLECTION,user_doc ).await {
        Ok(names) => Ok(names),
        Err(_) => Err(warp::reject()),
    }
}

pub async fn get_user(email: String) -> Result<Json, Rejection> {
    let filter = doc! { "Email": email };
    match find_user(USER_COLLECTION, filter).await {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(_) => Err(warp::reject()),
    }
}

async fn insert(collection_name: &str, document: bson::Document) -> Result<String, Error> {
    let result: String = String::new();
    let collection = get_collection(collection_name).await?;
    collection.insert_one(document, None).await?;
    Ok(result)
}

async fn find_user(collection_name: &str, filter: Document) -> Result<Document, Error> {

    let collection = get_collection(collection_name).await?;
    let mongo_result = collection.find_one(filter, None).await?;

    Ok(mongo_result.unwrap())
}

async fn get_collection(collection: &str) -> Result<Collection, Error>
{
    let client = common::initialize_mongo().await?;
    let db = client.database("blog");
    Ok(db.collection(collection))
}