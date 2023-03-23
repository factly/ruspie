use serde::Deserialize;
use worker::*;

use crate::context::OpenAIContext;
pub async fn handler(mut req: Request, ctx: RouteContext<OpenAIContext>) -> Result<Response> {
    let payload = req
        .json::<HandlerRequest>()
        .await
        .map_err(|e| Response::error(e.to_string(), 400))
        .unwrap();
    let data = ctx
        .data
        .generate(payload.prompt.into())
        .await
        .map(|data| data.choices[0].text.to_string())
        .unwrap();
    Response::ok(data)
}

#[derive(Debug, Deserialize)]
struct HandlerRequest {
    prompt: String,
}
