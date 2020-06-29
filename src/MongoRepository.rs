use mongodb::{Client, options::ClientOptions};
use warp::http;


pub fn init_mongo() -> str
{
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("db-blog".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    let collections = String::new();
    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        collections += db_name;
    }

    return collections;
}