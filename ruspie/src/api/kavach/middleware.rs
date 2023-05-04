use axum::{middleware::Next, response::Response};
use hyper::{Request, StatusCode};

use super::{KavachValidateToken, ValidateTokenTrait};

pub async fn kavach_middleware<B>(
    req: Request<B>,
    next: Next<B>,
    kavach: KavachValidateToken,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        let token = parse_auth_header(auth_header)?;
        let is_valid = kavach.validate_token(token).await;
        if !is_valid {
            return Err(StatusCode::FORBIDDEN);
        } else {
            return Ok(next.run(req).await);
        }
    }
    return Err(StatusCode::UNAUTHORIZED);
}

fn parse_auth_header(auth_header: &str) -> Result<&str, StatusCode> {
    let token = auth_header.splitn(2, " ").collect::<Vec<&str>>();
    if token.len() != 2 {
        return Err(StatusCode::FORBIDDEN);
    }

    return Ok(token[1]);
}
