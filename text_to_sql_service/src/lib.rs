use context::OpenAIContext;
use router::build_router;
use worker::*;

mod context;
mod models;
mod router;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let endpoint_url = std::env::var("ENDPOINT_URL").unwrap();
    let context = OpenAIContext::new(api_key, endpoint_url);
    let router = build_router(context);
    router.await.run(req, env).await
}
