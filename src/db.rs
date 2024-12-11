use mongodb::{Client, Database};
use mongodb::options::ClientOptions;

pub struct MongoDB {
    database: Database,
}

impl MongoDB {
    pub async fn new(uri: &str, db_name: &str) -> Self {
        let client_options = ClientOptions::parse(uri).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let database = client.database(db_name);

        Self { database }
    }

    pub fn get_database(&self) -> &Database {
        &self.database
    }
}
