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
pub struct JWTUserPayload {
    pub name: UserName,
    pub email: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: UserName,
    pub email: String,
    pub role: Role,
    pub password: String,
}

impl From<User> for JWTUserPayload {
    fn from(
        User {
            name, email, role, ..
        }: User,
    ) -> Self {
        Self {
            name: UserName {
                first: name.first,
                last: name.last,
            },
            email,
            role,
        }
    }
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
        page: Option<u32>,
        limit: Option<u32>,
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
                                {"$skip": (page - 1) * limit},
                                {"$limit": limit},
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
