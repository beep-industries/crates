use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};
use beep_auth::domain::{
    models::{AuthError, Token},
    ports::{AuthRepository, HasAuthRepository},
};
use tracing::{debug, error};

pub mod response;

#[derive(Debug)]
pub enum MiddlewareError {
    MissingAuthHeader,
    InvalidAuthHeader,
    AuthenticationFailed(AuthError),
}

impl From<MiddlewareError> for StatusCode {
    fn from(error: MiddlewareError) -> Self {
        match error {
            MiddlewareError::MissingAuthHeader => StatusCode::UNAUTHORIZED,
            MiddlewareError::InvalidAuthHeader => StatusCode::UNAUTHORIZED,
            MiddlewareError::AuthenticationFailed(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

pub async fn extract_token_from_bearer(auth_header: &HeaderValue) -> Result<Token, AuthError> {
    let auth_str = auth_header.to_str().map_err(|_| AuthError::TokenNotFound)?;

    if !auth_str.starts_with("Bearer ") {
        return Err(AuthError::TokenNotFound);
    }

    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or(AuthError::TokenNotFound)?;

    Ok(Token::new(token.to_string()))
}

pub async fn auth_middleware<T>(
    State(state): State<T>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode>
where
    T: HasAuthRepository + Send + Sync,
{
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or(MiddlewareError::MissingAuthHeader)?;

    let token = extract_token_from_bearer(auth_header)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let identity = state
        .auth_repository()
        .identify(token.as_str())
        .await
        .map_err(|e| {
            error!("auth middleware: failed to identity user {:?}", e);
            MiddlewareError::AuthenticationFailed(e)
        })?;

    debug!(
        "auth middleware: successfully identified user: {}",
        identity.id()
    );

    req.extensions_mut().insert(identity);

    Ok(next.run(req).await)
}
