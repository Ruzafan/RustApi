use crate::common;
use mongodb::error::Error;
use warp::Rejection;

pub async fn init_mongo() -> Result<String, Rejection> {
    match get_databases_names().await {
        Ok(names) => Ok(names),
        Err(_) => Err(warp::reject()),
    }
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
