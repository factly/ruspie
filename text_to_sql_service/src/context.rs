use crate::models::{OpenAIRequest, OpenAIResponse};

pub struct OpenAIContext {
    pub api_key: String,
    pub endpoint_url: String,
}

impl OpenAIContext {
    pub fn new(api_key: String, endpoint_url: String) -> Self {
        Self {
            endpoint_url,
            api_key,
        }
    }

    pub async fn generate(&self, payload: OpenAIRequest) -> OpenAIResponse {
        let client = reqwest::Client::new();
        client
            .post(self.endpoint_url.clone())
            .json(&payload)
            .send()
            .await
            .unwrap()
            .json::<OpenAIResponse>()
            .await
            .unwrap()
    }
}
