use crate::common;
use mongodb::error::Error;
use warp::Rejection;
use mongodb::bson::doc;

pub async fn init_mongo() -> Result<String, Rejection> {
    match get_databases_names().await {
        Ok(names) => Ok(names),
        Err(_) => Err(warp::reject()),
    }
}
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

async fn get_databases_names() -> Result<String, Error> {
    let client = common::initialize_mongo().await?;
    let mut collections = String::new();
    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        collections.push_str(&db_name)
    }

    Ok(collections)
}