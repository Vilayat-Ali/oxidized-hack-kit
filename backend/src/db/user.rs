use mongodb::{
    bson::Document,
    error::Result,
    results::{DeleteResult, InsertOneResult},
    Collection, Database,
};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: UserName,
    pub email: String,
    pub role: Role,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserName {
    pub first: String,
    pub last: String,
}

#[derive(Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

pub struct UserModel {
    collection: Collection<User>,
}

impl UserModel {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }

    pub async fn create(&self, user_instance: &User) -> Result<InsertOneResult> {
        self.collection.insert_one(user_instance, None).await
    }

    pub async fn delete(&self, doc: Document) -> Result<DeleteResult> {
        self.collection.delete_one(doc, None).await
    }
}
