use crate::{db::user::JWTUserPayload, utils::jwt::JWT};
use axum::{extract::Request, middleware::Next, response::Response};

pub async fn auth_middleware(request: Request, next: Next) -> Response {
    let token = request.headers().get("authorization");

    if let Some(jwt_token) = token {
        if jwt_token.is_empty() {
            // token not provided
        }

        match JWT::validate_token::<JWTUserPayload>(
            &jwt_token.to_str().unwrap_or_default().to_owned(),
        ) {
            Ok(user_data) => todo!(),
            Err(e) => todo!(),
        }
    };

    // headers not passed

    next.run(request).await
}
