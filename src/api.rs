use axum::{Extension, Router, routing::post};
use futures_util::stream::StreamExt;
use rig::{OneOrMany, completion::CompletionRequest, message::Message};
use std::sync::Arc;

use super::client::OllamaClient;

pub fn router() -> Router {
    Router::new()
        .route("/", post(receive_message))
        .layer(Extension(Arc::new(OllamaClient::new())))
}

async fn receive_message(Extension(client): Extension<Arc<OllamaClient>>) -> String {
    let request = CompletionRequest {
        preamble: Some(String::from("You are a humorous friend")),
        chat_history: OneOrMany::one(Message::user("Hi")),
        documents: Vec::new(),
        tools: Vec::new(),
        temperature: None,
        max_tokens: None,
        tool_choice: None,
        additional_params: None,
    };

    let result = client.stream(request).await;
    if let Ok(mut stream) = result {
        while let Some(token) = stream.next().await {
            print!("{:#?}", token);
        }
        String::from("Completed stream")
    } else {
        String::from("Receive Message")
    }
}
