use axum::extract::State;

use crate::Ctx;

pub async fn get_user(State(state): State<Ctx>) -> String {
    match state.lock().await.user.as_ref() {
        Some(user) => user.email.clone(),
        None => "No value".to_string(),
    }
}
