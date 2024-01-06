use std::sync::Arc;

use crate::{db::user::UserReqPayload, utils::jwt::JWT, AppState};
use axum::extract::{Json, State};
use serde_json::json;

use crate::{
    db::user::{User, UserModel},
    response,
    utils::hash::BcryptHash,
};

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(req_payload): Json<UserReqPayload>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    let user = User::from(req_payload.clone());
    match JWT::generate_token(&user) {
        Ok(access_token) => {
            let User {
                name,
                email,
                role,
                password,
            } = user;
            // save user to database
            match User::new(name.first, name.last, email, Some(role), password) {
                Ok(user_instance) => {
                    // store user in the database
                    let model = UserModel::new(&state.mongo_instance.db);
                    model.create(&user_instance).await.unwrap();

                    // return response
                    Ok(response! {
                        200,
                        "User Added Successfully!",
                        {
                            "full_name": format!("{} {}", &user_instance.name.first, &user_instance.name.last),
                            "name": &user_instance.name,
                            "email": &user_instance.email,
                            "role": &user_instance.role,
                            "access_token": access_token
                        }
                    })
                }
                Err(_) => Err(response! {
                    500,
                    "Failed to add user"
                }),
            }
        }
        Err(_) => Err(response! {
            500,
            "Failed to process user info"
        }),
    }
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(req_payload): Json<UserReqPayload>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    let user_model = UserModel::new(&state.mongo_instance.db);
    let user_password = req_payload.password.clone();
    match user_model.get_one(&req_payload.clone().email).await {
        Ok(user) => match user {
            Some(user) => {
                let User {
                    name,
                    email,
                    password,
                    role,
                } = user.clone();
                match BcryptHash::verify_hash(&password, user_password) {
                    Ok(result) => match result {
                        true => {
                            if let Ok(access_token) = JWT::generate_token(&user) {
                                return Ok(response! {
                                    200,
                                    "User Logged In Successfully!",
                                    {
                                        "full_name": format!("{} {}", &name.first, &name.last),
                                        "name": &user.name,
                                        "email": email,
                                        "role": role,
                                        "access_token": access_token
                                    }
                                });
                            } else {
                                Err(response! {500, "Internal Server Error"})
                            }
                        }
                        false => Err(response! {401, "Invalid `email` or `password`"}),
                    },
                    Err(_) => Err(response! {500, "User not found!"}),
                }
            }
            None => Err(response! {500, "User not found!"}),
        },
        Err(_) => Err(response! {500, "Internal Server Error"}),
    }
}
