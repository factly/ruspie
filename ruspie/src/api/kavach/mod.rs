#![allow(dead_code)]

pub mod middleware;

#[async_trait::async_trait]
pub trait ValidateTokenTrait {
    async fn validate_token(&self, token: &str) -> bool;
}

#[derive(Clone)]
pub struct KavachValidateToken {
    req: reqwest::Client,
}

impl KavachValidateToken {
    pub fn new(req: reqwest::Client) -> Self {
        Self { req }
    }
}

#[derive(serde::Serialize)]
struct KavachRequestBody {
    token: String,
}

#[async_trait::async_trait]
impl ValidateTokenTrait for KavachValidateToken {
    async fn validate_token(&self, token: &str) -> bool {
        let kavach_url = std::env::var("KAVACH_URL").unwrap_or("http://localhost:5001".to_string());
        let application_id = std::env::var("KAVACH_APPLICATION_ID")
            .unwrap_or_else(|_| panic!("KAVACH_APPLICATION_ID is required"))
            .parse::<u32>()
            .unwrap();
        let organization_id = std::env::var("KAVACH_ORGANISATION_ID")
            .unwrap_or_else(|_| panic!("KAVACH_ORGANISATION_ID is required"))
            .parse::<u32>()
            .unwrap();
        let kavach_url = format!(
            "{}/organisations/{}/applications/{}/tokens/validate",
            kavach_url, application_id, organization_id
        );
        let res = self
            .req
            .post(kavach_url)
            .header("X-Organisation", organization_id)
            .json(&KavachRequestBody {
                token: token.to_owned(),
            })
            .send()
            .await;
        if res.is_err() {
            return false;
        }
        if res.unwrap().status() != 200 {
            return false;
        }
        true
    }
}
