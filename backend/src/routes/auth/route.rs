use crate::{
    db::user::{self, UserName},
    utils::jwt::JWT,
};
use axum::extract::Json;
use serde::{Deserialize, Serialize};

use crate::db::user::{JWTUserPayload, User, UserModel};

#[derive(Serialize, Deserialize)]
struct AuthSuccessResponse {
    name: UserName,
    email: String,
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct AuthErrorResponse {
    status: u16,
    message: String,
}

pub async fn register_handler(
    Json(req_payload): Json<User>,
) -> Result<Json<AuthSuccessResponse>, Json<AuthErrorResponse>> {
    let password = req_payload.password.clone();
    match JWT::generate_token(&JWTUserPayload::from(req_payload)) {
        Ok(access_token) => {
            // save user to database
            unimplemented!();
            // forward a response
            unimplemented!();
        }
        Err(_) => Err(Json::from(AuthErrorResponse {
            status: 500,
            message: "Failed".to_owned(),
        })),
    }
}

pub async fn login_handler() -> &'static str {
    "login"
}
