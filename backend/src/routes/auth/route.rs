use crate::{
    db::user::{Role, UserName},
    utils::jwt::JWT,
};
use axum::extract::Json;
use serde::{Deserialize, Serialize};

use crate::db::user::{JWTUserPayload, User};

#[derive(Serialize, Deserialize)]
pub struct AuthSuccessResponse {
    name: UserName,
    email: String,
    role: Role,
    access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthErrorResponse {
    status: u16,
    message: String,
}

pub async fn register_handler(
    Json(req_payload): Json<User>,
) -> Result<Json<AuthSuccessResponse>, Json<AuthErrorResponse>> {
    match JWT::generate_token(&JWTUserPayload::from(req_payload.clone())) {
        Ok(access_token) => {
            let User {
                name,
                email,
                role,
                password,
            } = req_payload;
            // save user to database
            match User::new(name.first, name.last, email, Some(role), password) {
                Ok(User {
                    name,
                    email,
                    role,
                    password,
                }) => {
                    // store user in the database
                    unimplemented!();

                    // return response
                    return Ok(Json(AuthSuccessResponse {
                        name,
                        email,
                        role,
                        access_token,
                    }));
                }
                Err(_) => {
                    return Err(Json(AuthErrorResponse {
                        status: 500,
                        message: "Failed to hash user info".to_owned(),
                    }));
                }
            }
        }
        Err(_) => Err(Json::from(AuthErrorResponse {
            status: 500,
            message: "Failed to process request data".to_owned(),
        })),
    }
}

pub async fn login_handler() -> &'static str {
    "login"
}
