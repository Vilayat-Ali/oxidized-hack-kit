mod route;

use crate::Ctx;
use axum::{routing::get, Router};
use route::get_user;

pub struct UserRoutes;

impl UserRoutes {
    pub fn get_routes() -> Router<Ctx> {
        Router::new().route("/", get(get_user))
    }
}
