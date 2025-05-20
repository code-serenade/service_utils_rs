use std::sync::Arc;

use axum::{
    extract::Request,
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::services::jwt::Jwt;

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let token = parse_token(headers)?;
    let jwt = req
        .extensions()
        .get::<Arc<Jwt>>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let claims = jwt
        .validate_access_token(&token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}

fn parse_token(headers: &HeaderMap) -> Result<String, StatusCode> {
    let authorization = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?;

    let mut parts = authorization.to_str().unwrap().splitn(2, ' ');
    match parts.next() {
        Some(scheme) if scheme == "Bearer" => {}
        _ => return Err(StatusCode::UNAUTHORIZED),
    }

    let token = parts.next().ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(token.to_string())
}
