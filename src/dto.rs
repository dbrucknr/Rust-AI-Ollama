use serde::Deserialize;

#[derive(Deserialize)]
pub struct IncomingMessage {
    pub content: String,
}
