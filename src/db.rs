use mongodb::{Client, Database};
use mongodb::options::ClientOptions;

pub async fn get_database(uri: &str) -> Database {
    let client_options = ClientOptions::parse(uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database("url_shortner")

}