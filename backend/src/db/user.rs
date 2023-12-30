use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
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
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }

    pub async fn create(&self, user_instance: &User) -> Result<InsertOneResult> {
        self.collection.insert_one(user_instance, None).await
    }

    pub async fn get_paginated_data(
        &self,
        search: Option<String>,
        page: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Vec<Document>> {
        let search = search.unwrap_or_default();
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(20);

        let cursor = self
            .collection
            .aggregate(
                vec![
                    doc! {
                        "email": {
                            "$regex": search,
                            "$options": "i"
                        }
                    },
                    doc! {
                        "$match": {
                            "$and": [

                            ]
                        }
                    },
                    doc! {
                        "$facet": {
                            "data": [
                                {"$skip": ((page - 1) * limit).to_string()},
                                {"$limit": limit.to_string()},
                            ],
                            "count": [
                                {
                                    "$count": "count"
                                }
                            ]
                        }
                    },
                ],
                None,
            )
            .await?;
        let result: Vec<Document> = cursor.try_collect().await?;
        Ok(result)
    }

    pub async fn update(&self, doc: Document, updated_doc: Document) -> Result<Option<User>> {
        self.collection
            .find_one_and_update(doc, updated_doc, None)
            .await
    }

    pub async fn delete(&self, doc: Document) -> Result<DeleteResult> {
        self.collection.delete_one(doc, None).await
    }
}
