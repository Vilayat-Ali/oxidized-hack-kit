use crate::{
    db::user::{JWTUserPayload, UserReqPayload},
    utils::jwt::JWT,
    Ctx,
};
use axum::extract::{Json, State};
use serde_json::json;

use crate::{
    db::user::{User, UserModel},
    response,
};

pub async fn register_handler(
    State(state): State<Ctx>,
    Json(req_payload): Json<UserReqPayload>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    let user = User::from(req_payload.clone());
    let model = UserModel::new(&state.lock().await.mongo_instance.db);

    if let Ok(prob_user) = model.get_one(&req_payload.email).await {
        if prob_user.is_some() {
            return Err(response!(403, "User with same email already exists!"));
        }
    };

    let token_payload = JWTUserPayload::new(user.name.clone(), user.email.clone(), None);

    if let Ok(access_token) = JWT::generate_token(&token_payload) {
        let User {
            name,
            email,
            role,
            password,
        } = user;
        // save user to database
        if let Ok(user_instance) = User::new(name.first, name.last, email, Some(role), password) {
            // store user in the database
            model.create(&user_instance).await.unwrap();

            // return response
            return Ok(response! {
                200,
                "User Added Successfully!",
                {
                    "full_name": format!("{} {}", &user_instance.name.first, &user_instance.name.last),
                    "name": &user_instance.name,
                    "email": &user_instance.email,
                    "role": &user_instance.role,
                    "access_token": access_token
                }
            });
        };
    }

    Err(response! {
        500,
        "Failed to process user info"
    })
}

pub async fn login_handler(State(_state): State<Ctx>, Json(_req_payload): Json<UserReqPayload>) {
    // let user_model = UserModel::new(&state.mongo_instance.db);
    // let user_password = req_payload.password.clone();
    // match user_model.get_one(&req_payload.clone().email).await {
    //     Ok(user) => match user {
    //         Some(user) => {
    //             let User {
    //                 name,
    //                 email,
    //                 password,
    //                 role,
    //             } = user.clone();
    //             match BcryptHash::verify_hash(&password, user_password) {
    //                 Ok(result) => match result {
    //                     true => {
    //                         if let Ok(access_token) = JWT::generate_token(&user) {
    //                             return Ok(response! {
    //                                 200,
    //                                 "User Logged In Successfully!",
    //                                 {
    //                                     "full_name": format!("{} {}", &name.first, &name.last),
    //                                     "name": &user.name,
    //                                     "email": email,
    //                                     "role": role,
    //                                     "access_token": access_token
    //                                 }
    //                             });
    //                         } else {
    //                             Err(response! {500, "Internal Server Error"})
    //                         }
    //                     }
    //                     false => Err(response! {401, "Invalid `email` or `password`"}),
    //                 },
    //                 Err(_) => Err(response! {500, "User not found!"}),
    //             }
    //         }
    //         None => Err(response! {500, "User not found!"}),
    //     },
    //     Err(_) => Err(response! {500, "Internal Server Error"}),
    // }
}
