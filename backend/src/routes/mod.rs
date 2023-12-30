use axum::{routing::get, Router};

pub struct ApiRoutes;

impl ApiRoutes {
    pub fn get_routes() -> Router {
        Router::new()
    }
}
