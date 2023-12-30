use axum::{routing::get, Router};

// routes
mod auth;
mod user;

use auth::AuthRoutes;
use user::UserRoutes;

pub struct ApiRoutes;

impl ApiRoutes {
    pub fn get_routes() -> Router {
        Router::new()
            .nest("/auth", AuthRoutes::get_routes())
            .nest("/user", UserRoutes::get_routes())
    }
}
