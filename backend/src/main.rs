use axum::Router;
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
