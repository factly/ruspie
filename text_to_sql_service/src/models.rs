use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIRequest {
    model: String,
    pub prompt: String,
    temperature: u64,
    max_tokens: u64,
    stop: Vec<String>,
}

impl From<String> for OpenAIRequest {
    fn from(prompt: String) -> Self {
        let stop: Vec<String> = vec!["#".to_owned(), ";".to_owned()];
        Self {
            model: String::from("code-davinci-002"),
            prompt,
            temperature: 0,
            max_tokens: 150,
            stop,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIResponse {
    choice: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub text: String,
    pub index: u64,
    pub finished_reason: String,
}
