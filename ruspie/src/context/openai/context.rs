use hyper::StatusCode;
use models::{OpenAIRequest, OpenAIResponse};

use super::models::{self, OpenAIAPIRequestError};

#[derive(Clone, Debug)]
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
    ) -> Result<OpenAIResponse, OpenAIAPIRequestError> {
        let client = reqwest::Client::new();
        let res = client
            .post(self.endpoint_url.clone())
            .json(&payload)
            .header("AUTHORIZATION", format!("Bearer {}", &self.api_key.clone()))
            .send()
            .await
            .map_err(|e| OpenAIAPIRequestError {
                message: e.to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            })?
            .json::<OpenAIResponse>()
            .await
            .map_err(|e| OpenAIAPIRequestError {
                message: e.to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR,
            });
        res
    }
}
