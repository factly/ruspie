use serde::Deserialize;
use worker::*;

use crate::context::OpenAIContext;
pub async fn handler(mut req: Request, ctx: RouteContext<OpenAIContext>) -> Result<Response> {
    let request_payload = req.json::<HandlerRequest>().await;
    if let Err(e) = request_payload {
        return Response::error(e.to_string(), 400);
    }
    let payload = request_payload.unwrap();
    let generate_result = ctx.data.generate(payload.prompt.into()).await;
    if let Err(err) = generate_result {
        return err.into();
    }
    Response::ok(&generate_result.unwrap().choices[0].text)
}

#[derive(Debug, Deserialize)]
struct HandlerRequest {
    prompt: String,
}
