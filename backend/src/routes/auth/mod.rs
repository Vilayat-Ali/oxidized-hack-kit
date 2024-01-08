mod route;

use crate::Ctx;
use axum::{routing::post, Router};
use route::{login_handler, register_handler};

pub struct AuthRoutes;

impl AuthRoutes {
    pub fn get_routes() -> Router<Ctx> {
        Router::new()
            .route("/register", post(register_handler))
            .route("/login", post(login_handler))
    }
}
