use std::sync::Arc;

use axum::{routing::post, Router};

mod route;

use route::{login_handler, register_handler};

use crate::AppState;

pub struct AuthRoutes;

impl AuthRoutes {
    pub fn get_routes() -> Router<Arc<AppState>> {
        Router::new()
            .route("/register", post(register_handler))
            .route("/login", post(login_handler))
    }
}
