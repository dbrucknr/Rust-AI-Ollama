use axum::{Extension, Router, routing::post};
use std::sync::Arc;

use super::client::OllamaClient;

pub fn router() -> Router {
    Router::new()
        .route("/", post(receive_message))
        .layer(Extension(Arc::new(OllamaClient::new())))
}

async fn receive_message() -> String {
    String::from("Receive Message")
}
