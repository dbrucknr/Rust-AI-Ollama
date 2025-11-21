use rig::{
    client::CompletionClient,
    completion::{CompletionError, CompletionModel, CompletionRequest},
    providers::ollama::{Client, StreamingCompletionResponse as OllamaStream},
    streaming::StreamingCompletionResponse,
};
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
