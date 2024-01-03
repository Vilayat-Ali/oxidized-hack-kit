use axum::Router;
use std::io;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use backend::{db::user::JWTUserPayload, db::Mongo, routes::ApiRoutes, ENV};

#[derive(Clone, Debug)]
pub struct AppState {
    pub mongo_instance: Mongo,
    pub user: Option<JWTUserPayload>,
}

impl AppState {
    fn new(mongo_instance: Mongo) -> Self {
        Self {
            mongo_instance,
            user: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // loading envs
    let ENV { port, .. } = ENV::import();

    // tracing and logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // database connection establishment
    let mongo_ctx = Mongo::establish_connection()
        .await
        .expect("Failed to connect to database...");

    // setting app state
    let ctx = Arc::new(AppState::new(mongo_ctx));

    // middlewares
    let cors = CorsLayer::new().allow_origin(Any);
    let compression = CompressionLayer::new().gzip(true);
    let trace = TraceLayer::new_for_http();

    let middlewares = ServiceBuilder::new()
        .layer(compression)
        .layer(trace)
        .layer(cors);

    // Router
    let app = Router::new()
        .nest("/api", ApiRoutes::get_routes())
        .with_state(())
        .layer(middlewares);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
