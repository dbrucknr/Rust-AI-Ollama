// Standard Library Crates
use std::{error::Error, net::SocketAddr};

// Third Party Library Crates
use axum::{Router, serve};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use crate::api::router;

// Local Module Registry
pub mod api;
pub mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);
    let app = Router::new().nest("/api", router()).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = TcpListener::bind(addr).await?;

    serve(listener, app).await?;

    Ok(())
}
