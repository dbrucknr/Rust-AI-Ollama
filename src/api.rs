use axum::{
    Extension, Json, Router,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
// use futures::Stream;
use rig::{
    OneOrMany,
    completion::{CompletionRequest, CompletionResponse},
    message::Message,
    providers::ollama::{
        AssistantContent, CompletionResponse as OllamaResponse, Message as OllamaMessage,
    },
};
use std::sync::Arc;
// use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use super::client::OllamaClient;

pub fn router() -> Router {
    Router::new()
        .route("/", post(respond_to_message))
        .layer(Extension(Arc::new(OllamaClient::new())))
}

pub enum ApiError {
    InternalServerError,
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Something unexpected happened"),
            ),
        }
        .into_response()
    }
}

async fn respond_to_message(
    Extension(client): Extension<Arc<OllamaClient>>,
) -> Result<String, ApiError> {
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
    if let Ok(reply) = client.generate_reply(request).await {
        let response_content = match reply.raw_response.message {
            OllamaMessage::User { content, .. } => content,
            OllamaMessage::Assistant { content, .. } => content,
            OllamaMessage::System { content, .. } => content,
            OllamaMessage::ToolResult { content, .. } => content,
        };
        Ok(response_content)
    } else {
        Err(ApiError::InternalServerError)
    }
}

// async fn stream_response(
//     Extension(client): Extension<Arc<OllamaClient>>,
// ) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
//     let request = CompletionRequest {
//         preamble: Some(String::from("You are a humorous friend")),
//         chat_history: OneOrMany::one(Message::user("Hi")),
//         documents: Vec::new(),
//         tools: Vec::new(),
//         temperature: None,
//         max_tokens: None,
//         tool_choice: None,
//         additional_params: None,
//     };

//     let result = client.stream(request).await;
//     Sse::new(stream).keep_alive(KeepAlive::default())
// }
