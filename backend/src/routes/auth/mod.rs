use axum::{routing::post, Router};

mod route;

use route::{login_handler, register_handler};

pub struct AuthRoutes;

impl AuthRoutes {
    pub fn get_routes() -> Router {
        Router::new()
            .route("/register", post(register_handler()))
            .route("/login", post(login_handler()))
    }
}
