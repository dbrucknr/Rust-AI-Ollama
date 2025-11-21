use futures_util::stream::StreamExt;

use rig::message::Message;
use rig::{
    OneOrMany,
    client::CompletionClient,
    completion::{CompletionError, CompletionModel, CompletionRequest, Document},
    providers::ollama::{Client, StreamingCompletionResponse as OllamaStream},
    streaming::StreamingCompletionResponse,
};
// use rig::providers::ollama::Client;

pub struct OllamaClient {
    client: Client,
    model_name: String,
}
impl OllamaClient {
    pub fn new() -> Self {
        Self {
            client: Client::default(), // Uses http://localhost:11434
            model_name: String::from("mistral"),
        }
    }

    pub async fn stream(
        &self,
        request: CompletionRequest,
    ) -> Result<StreamingCompletionResponse<OllamaStream>, CompletionError> {
        let model = self.client.completion_model(&self.model_name);
        model.stream(request).await
    }
}

// let request = CompletionRequest {
//     preamble: Some(String::from("You are a humorous friend")),
//     chat_history: OneOrMany::one(Message::user("Hi")),
//     documents: Vec::<Document>::new(),
//     tools: Vec::new(),
//     temperature: None,
//     max_tokens: None,
//     tool_choice: None,
//     additional_params: None,
// };

// let mut stream = model.stream(request).await?;

// while let Some(token) = stream.next().await {
//     print!("{:#?}", token?);
// }
