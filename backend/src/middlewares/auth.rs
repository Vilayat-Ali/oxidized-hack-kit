use crate::{db::user::JWTUserPayload, utils::jwt::JWT, Ctx};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(State(state): State<Ctx>, request: Request, next: Next) -> Response {
    let token = request.headers().get("Authorization");

    if let Some(jwt_token) = token {
        let token = jwt_token
            .to_str()
            .unwrap_or_default()
            .to_owned()
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        match JWT::validate_token::<JWTUserPayload>(&token[1]) {
            Ok(user_data) => {
                {
                    let mut user_in_state = state.lock().await;
                    user_in_state.user = Some(user_data);
                }
                return next.run(request).await;
            }
            Err(_e) => {
                return Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body("Auth token expired or invalid".into())
                    .unwrap();
            }
        }
    }

    return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("`Authorization` headers not set before request was sent".into())
        .unwrap();
}
