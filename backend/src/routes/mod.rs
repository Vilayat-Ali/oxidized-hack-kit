use axum::{middleware, Router};

// routes
mod auth;
mod user;

use crate::{middlewares::auth::auth_middleware, Ctx};
use auth::AuthRoutes;
use user::UserRoutes;

pub struct ApiRoutes;

impl ApiRoutes {
    pub fn get_routes(ctx: Ctx) -> Router<Ctx> {
        Router::new().nest("/auth", AuthRoutes::get_routes()).nest(
            "/user",
            UserRoutes::get_routes()
                .layer(middleware::from_fn_with_state(ctx.clone(), auth_middleware)),
        )
    }
}
