use context::OpenAIContext;
use worker::*;

mod context;
mod handler;
mod models;
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
    dotenvy::dotenv().ok();
    log_request(&req);
    utils::set_panic_hook();
    let api_key = env.var("OPENAI_API_KEY").unwrap().to_string();
    let endpoint_url = env.var("ENDPOINT_URL").unwrap().to_string();
    let context = OpenAIContext::new(api_key, endpoint_url);
    let router = Router::with_data(context);
    router
        .get("/", |_, _| Response::ok("Hello from workers"))
        .post_async("/generate", handler::handler)
        .run(req, env)
        .await
}
