use axum::{
    Extension, Json, Router,
    http::StatusCode,
    response::{
        IntoResponse, Response, Sse,
        sse::{Event, KeepAlive},
    },
    routing::{get, post},
};
use futures::Stream;
use rig::{
    OneOrMany, completion::CompletionRequest, message::Message,
    providers::ollama::Message as OllamaMessage, streaming::StreamedAssistantContent,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio_stream::StreamExt;

use super::client::OllamaClient;

pub fn router() -> Router {
    Router::new()
        .route("/", get(stream_response))
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

#[derive(Deserialize)]
struct IncomingMessage {
    content: String,
}

async fn respond_to_message(
    Extension(client): Extension<Arc<OllamaClient>>,
    Json(body): Json<IncomingMessage>,
) -> Result<String, ApiError> {
    let request = CompletionRequest {
        preamble: Some(String::from("You are a helpful and encouraging friend")),
        chat_history: OneOrMany::one(Message::user(body.content)),
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

async fn stream_response(
    Extension(client): Extension<Arc<OllamaClient>>,
) -> Result<Sse<impl Stream<Item = Result<Event, axum::Error>>>, ApiError> {
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

    let result = client
        .stream(request)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    let sse_stream = result.map(|chunk_result| {
        match chunk_result {
            Ok(chunk) => {
                let response_content = match chunk {
                    StreamedAssistantContent::Text(text) => text.text,
                    StreamedAssistantContent::Final(_) => String::from("[done]"),
                    _ => "".to_string(), // Skip ToolCall, Reasoning, etc.
                };
                // This could be moved into each match with a custom payload
                Ok(Event::default().data(response_content))
            }
            Err(_e) => Ok(Event::default().data("[error]")),
        }
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}
