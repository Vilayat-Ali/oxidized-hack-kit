use axum::{
    routing::{get, put},
    Router,
};

mod route;

use route::login_handler;

pub struct UserRoutes;

impl UserRoutes {
    pub fn get_routes() -> Router {
        Router::new()
            .route("/", get(login_handler))
            .route("/", get(login_handler))
            .route("/", put(login_handler))
    }
}
