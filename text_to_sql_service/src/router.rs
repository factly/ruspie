use crate::context::OpenAIContext;
use worker::*;
pub async fn build_router(_context: OpenAIContext) -> Router<'static, ()> {
    let router = Router::new();

    router.get("/", |_, _| Response::ok("hello there!"))
}
