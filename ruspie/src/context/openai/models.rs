use axum::response::IntoResponse;
use hyper::http;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
}

impl From<String> for OpenAIRequest {
    fn from(content: String) -> Self {
        let messages = vec![Message {
            role: "user".to_string(),
            content,
        }];
        OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<Choice>,
}

impl From<reqwest::Response> for OpenAIResponse {
    fn from(value: reqwest::Response) -> Self {
        let res: OpenAIResponse = value.into();
        res
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: u64,
    pub message: Message,
}

#[derive(Debug, Serialize)]
pub struct OpenAIAPIRequestError {
    pub message: String,
    #[serde(serialize_with = "serialize_statuscode")]
    pub status: http::StatusCode,
}

fn serialize_statuscode<S>(x: &http::StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(x.as_u16())
}

impl IntoResponse for OpenAIAPIRequestError {
    fn into_response(self) -> axum::response::Response {
        let payload = serde_json::to_string(&self).unwrap();
        let body = axum::body::boxed(axum::body::Full::from(payload));

        axum::response::Response::builder()
            .status(self.status)
            .body(body)
            .unwrap()
    }
}
