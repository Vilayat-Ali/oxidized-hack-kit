use axum::Router;
use std::io;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use backend::{db::user::JWTUserPayload, db::Mongo, routes::ApiRoutes, AppState, ENV};

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
    let cors = CorsLayer::new()
        .allow_headers(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any);
    let compression = CompressionLayer::new().gzip(true);
    let trace = TraceLayer::new_for_http();

    // Router
    let app = Router::new()
        .nest("/api", ApiRoutes::get_routes())
        .layer(cors)
        .layer(trace)
        .layer(compression)
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
