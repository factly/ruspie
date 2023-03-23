use serde::Deserialize;
use worker::*;

use crate::context::OpenAIContext;
pub async fn handler(mut req: Request, ctx: RouteContext<OpenAIContext>) -> Result<Response> {
    let payload = match req.json::<HandlerRequest>().await {
        Ok(payload) => payload,
        Err(e) => return Response::error(e.to_string(), 400),
    };
    match ctx.data.generate(payload.prompt.into()).await {
        Ok(res) => Response::ok(&res.choices[0].text),
        Err(e) => e,
    }
}

#[derive(Debug, Deserialize)]
struct HandlerRequest {
    prompt: String,
}
