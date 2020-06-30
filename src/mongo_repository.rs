use mongodb::{error::Error, options::ClientOptions, Client};
use warp::Rejection;

pub async fn init_mongo() -> Result<String, Rejection> {
    match get_databases_names().await {
        Ok(names) => Ok(names),
        Err(_) => Err(warp::reject()),
    }
}

async fn get_databases_names() -> Result<String, Error> {
    let client = initialize_mongo().await?;

    let mut collections = String::new();
    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        collections.push_str(&db_name)
    }

    Ok(collections)
}

async fn initialize_mongo() -> Result<Client, Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:example@localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("db-blog".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options);

    client
}
