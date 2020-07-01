use mongodb::{error::Error, options::ClientOptions, Client};

pub async fn initialize_mongo() -> Result<Client, Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("db-blog".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options);

    client
}
