use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::*;

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
#[derive(Debug, Serialize, Deserialize)]
struct OpenAIRequest {
    model: String,
    prompt: String,
    temperature: u64,
    max_tokens: u64,
    stop: Vec<String>,
}

impl Default for OpenAIRequest {
    fn default() -> Self {
        let mut stop: Vec<String> = Vec::new();
        stop.push("#".to_owned());
        stop.push(";".to_owned());
        Self {
            model: String::from("code-davinci-002"),
            prompt: String::from(""),
            temperature: 0,
            max_tokens: 150,
            stop,
        }
    }
}
#[derive(Debug, Deserialize)]
struct HandlerRequest {
    prompt: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    text: String,
    index: u64,
    finished_reason: String,
}
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .post_async("/", |mut req, _| async move {
            let body = req.json::<HandlerRequest>().await;
            if let Ok(body) = body {
                let mut openai_request = OpenAIRequest::default();
                openai_request.prompt = body.prompt;
                let res = reqwest::Client::new();
                let res = res
                    .post("https://api.openai.com/v1/completions")
                    .header(
                        "Authorization",
                        format!(
                            "Bearer {}",
                            "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"
                        ),
                    )
                    .json(&openai_request)
                    .send()
                    .await
                    .unwrap();
                let res = res.text().await.unwrap();
                return Response::ok(res);
            }
            Response::ok("bad request")
        })
        .run(req, env)
        .await
}
