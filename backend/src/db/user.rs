use crate::utils::hash::BcryptHash;
use chrono::{Duration, Utc};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    error,
    results::{DeleteResult, InsertOneResult},
    Collection, Database,
};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JWTUserPayload {
    pub name: UserName,
    pub email: String,
    pub role: Role,
    pub exp: usize,
}

impl JWTUserPayload {
    pub fn new(name: UserName, email: String, role: Option<Role>) -> Self {
        let current_time = Utc::now();
        Self {
            name,
            email,
            role: role.unwrap_or_default(),
            exp: (current_time + Duration::days(7)).timestamp() as usize,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: UserName,
    pub email: String,
    pub role: Role,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserReqPayload {
    pub name: UserName,
    pub email: String,
    pub role: Option<Role>,
    pub password: String,
}

impl From<UserReqPayload> for User {
    fn from(value: UserReqPayload) -> Self {
        Self {
            name: value.name,
            role: value.role.unwrap_or_default(),
            email: value.email,
            password: value.password,
        }
    }
}

impl User {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        role: Option<Role>,
        password: String,
    ) -> Result<Self, bcrypt::BcryptError> {
        let hashed_password = BcryptHash::hash_string(password)?;
        Ok(Self {
            name: UserName {
                first: first_name,
                last: last_name,
            },
            email,
            role: role.unwrap_or_default(),
            password: hashed_password,
        })
    }
}

impl From<User> for JWTUserPayload {
    fn from(val: User) -> Self {
        let current_time = Utc::now();
        JWTUserPayload {
            name: val.name,
            email: val.email,
            role: val.role,
            exp: (current_time + Duration::days(7)).timestamp() as usize,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserName {
    pub first: String,
    pub last: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    pub async fn create(&self, user_instance: &User) -> error::Result<InsertOneResult> {
        self.collection.insert_one(user_instance, None).await
    }

    pub async fn get_one(&self, email: &String) -> error::Result<Option<User>> {
        self.collection
            .find_one(
                doc! {
                    "email": email
                },
                None,
            )
            .await
    }

    pub async fn get_paginated_data(
        &self,
        search: Option<String>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> error::Result<Vec<Document>> {
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

    pub async fn update(
        &self,
        doc: Document,
        updated_doc: Document,
    ) -> error::Result<Option<User>> {
        self.collection
            .find_one_and_update(doc, updated_doc, None)
            .await
    }

    pub async fn delete(&self, doc: Document) -> error::Result<DeleteResult> {
        self.collection.delete_one(doc, None).await
    }
}
