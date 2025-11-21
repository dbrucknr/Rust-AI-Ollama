use std::error::Error;

use futures_util::stream::StreamExt;

use rig::OneOrMany;
use rig::client::CompletionClient;
use rig::completion::{CompletionModel, CompletionRequest, Document};
use rig::message::Message;
use rig::providers::ollama::Client;

// https://docs.rs/rig-core/latest/rig/providers/ollama/index.html#example

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::default(); // Uses http://localhost:11434 (Ollama must be running until Docker image is provided)

    let model = client.completion_model("mistral");

    let request = CompletionRequest {
        preamble: Some(String::from("You are a humorous friend")),
        chat_history: OneOrMany::one(Message::user("Hi")),
        documents: Vec::<Document>::new(),
        tools: Vec::new(),
        temperature: None,
        max_tokens: None,
        tool_choice: None,
        additional_params: None,
    };

    let mut stream = model.stream(request).await?;

    while let Some(token) = stream.next().await {
        print!("{:#?}", token?);
    }
    Ok(())
}
