use crate::models::{OpenAIRequest, OpenAIResponse};

pub struct OpenAIContext {
    api_key: String,
    endpoint_url: String,
}

impl OpenAIContext {
    pub fn new(api_key: String, endpoint_url: String) -> Self {
        Self {
            endpoint_url,
            api_key,
        }
    }

    pub async fn generate(
        &self,
        payload: OpenAIRequest,
    ) -> Result<OpenAIResponse, Result<worker::Response, worker::Error>> {
        let client = reqwest::Client::new();
        client
            .post(self.endpoint_url.clone())
            .json(&payload)
            .header("AUTHORIZATION", format!("Bearer {}", &self.api_key.clone()))
            .send()
            .await
            .map_err(|e| worker::Response::error(e.to_string(), 500))?
            .json::<OpenAIResponse>()
            .await
            .map_err(|e| worker::Response::error(e.to_string(), 500))
    }
}
