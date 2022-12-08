use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use regex::Regex;

use crate::context::auth::context::{AuthContext, RawAuthContext};

pub async fn auth_middleware<B>(
    req: Request<B>,
    next: Next<B>,
    auth: RawAuthContext,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if let Some(auth_header) = auth_header {
        let token = parse_auth_header(auth_header)?;
        let re = Regex::new(r"/keys*").unwrap();
        match re.is_match(req.uri().path().to_owned().as_str()) {
            true => {
                if token != auth.get_master_key().unwrap() {
                    return Err(StatusCode::FORBIDDEN);
                }
                return Ok(next.run(req).await);
            }
            false => {
                
                let s = auth
                    .get_optional_uid_from_encoded_key(token.as_bytes())
                    .map_err(|_e| StatusCode::FORBIDDEN)?;
                if s.is_none() {
                    return Err(StatusCode::FORBIDDEN);
                }
               return Ok(next.run(req).await);
            }
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
