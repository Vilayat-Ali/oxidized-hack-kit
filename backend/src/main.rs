use axum::Router;
use std::io;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use backend::routes::ApiRoutes;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // tracing and logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // middlewares
    let cors = CorsLayer::new().allow_origin(Any);
    let compression = CompressionLayer::new().gzip(true);
    let trace = TraceLayer::new_for_http();

    let middlewares = ServiceBuilder::new()
        .layer(compression)
        .layer(trace)
        .layer(cors);

    let app = Router::new()
        .nest("/api", ApiRoutes::get_routes())
        .layer(middlewares);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
