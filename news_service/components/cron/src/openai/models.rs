use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CompletionArgs {
    pub prompt: String,
    pub temperature: f32,
    pub max_tokens: u16,
    // pub stop: Vec<String>,
    pub model: String,
}

#[derive(Deserialize)]
pub struct Completion {
    pub id: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
}

#[derive(Deserialize)]
pub struct CompletionChoice {
    pub text: String,
    pub index: u64,
    pub finish_reason: String,
}
