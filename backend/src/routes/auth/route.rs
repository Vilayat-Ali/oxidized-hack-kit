use std::sync::Arc;

use crate::{
    db::user::{Role, UserName, UserReqPayload},
    utils::jwt::JWT,
    AppState,
};
use axum::extract::{Json, State};
use serde::{Deserialize, Serialize};

use crate::db::user::{User, UserModel};

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
    State(state): State<Arc<AppState>>,
    Json(req_payload): Json<UserReqPayload>,
) -> Result<Json<AuthSuccessResponse>, Json<AuthErrorResponse>> {
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
                    return Ok(Json(AuthSuccessResponse {
                        name: user_instance.name.clone(),
                        email: user_instance.email.clone(),
                        role: user_instance.role.clone(),
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
