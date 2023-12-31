use axum::Router;

pub struct UserRoutes;

impl UserRoutes {
    pub fn get_routes() -> Router {
        Router::new()
    }
}
