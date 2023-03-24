use crate::context::OpenAIContext;
use serde::{Deserialize, Serialize};
use worker::*;

pub async fn handler(mut req: Request, ctx: RouteContext<OpenAIContext>) -> Result<Response> {
    let request_payload = req.json::<HandlerRequest>().await;
    if let Err(e) = request_payload {
        return Response::error(e.to_string(), 400);
    }
    let payload = request_payload.unwrap();
    let generate_result = ctx.data.generate(payload.prompt.into()).await;
    if let Err(err) = generate_result {
        console_log!("{:?}", err);
        return err.into();
    }
    let res: HandlerResponse = HandlerResponse {
        response: generate_result.unwrap().choices[0]
            .message
            .content
            .to_string(),
    };
    let res = serde_json::to_string(&res);
    Response::ok(res.unwrap()).unwrap().with_cors(
        &Cors::new()
            .with_origins(vec!["*"])
            .with_methods(vec![Method::Post, Method::Get, Method::Options])
            .with_allowed_headers(vec!["Content-Type"]),
    )
}

pub fn preflight_response(headers: &Headers, cors_origin: &str) -> Result<Response> {
    let mut headers = worker::Headers::new();
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    headers.set("Access-Control-Allow-Methods", "POST")?;
    headers.set("Vary", "Origin")?;
    headers.set("Access-Control-Allow-Origin", "*")?;
    Ok(Response::empty()
        .unwrap()
        .with_headers(headers)
        .with_status(204))
}

#[derive(Debug, Serialize, Deserialize)]
struct HandlerResponse {
    pub response: String,
}

#[derive(Debug, Deserialize)]
struct HandlerRequest {
    prompt: String,
}

impl HandlerRequest {
    pub fn refactor_prompt(&self) -> String {
        todo!()
    }
}
