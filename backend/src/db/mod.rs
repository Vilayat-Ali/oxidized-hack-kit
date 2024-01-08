pub mod user;

use mongodb::{error::Result, options::ClientOptions, Client, Database};


use crate::ENV;

#[derive(Debug, Clone)]
pub struct Mongo {
    pub db: Database,
}

impl Mongo {
    pub async fn establish_connection() -> Result<Self> {
        let ENV { mongo_uri, .. } = ENV::import();
        let client_options = ClientOptions::parse(mongo_uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.default_database().unwrap();
        tracing::info!("Connected to MongoDB...");
        Ok(Self { db })
    }
}
