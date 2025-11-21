// Standard Library Crates
use std::{error::Error, net::SocketAddr};

// Third Party Library Crates
use axum::{Router, serve};
use tokio::net::TcpListener;

// Local Module Registry
pub mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = TcpListener::bind(addr).await?;

    serve(listener, app).await?;

    Ok(())
}
