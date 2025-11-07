use std::net::{SocketAddr, ToSocketAddrs};

use axum::{Json, Router, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;
use tracing::{error, info};

pub mod args;
pub mod config;
pub mod http;

pub async fn get_addr(host: &str, port: u16) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let addrs = format!("{}:{}", host, port)
        .to_socket_addrs()?
        .collect::<Vec<SocketAddr>>();

    let socket = match addrs.first() {
        Some(addr) => *addr,
        None => return Err("No socket addresses found".into()),
    };

    Ok(socket)
}

pub async fn run_server(addr: SocketAddr, router: Router) {
    info!("listening on {addr}");

    if let Err(e) = axum_server::bind(addr)
        .serve(router.into_make_service())
        .await
    {
        error!("server error: {}", e);
        std::process::exit(1);
    }
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("unknown error occurred: {message}")]
    Unknown { message: String },

    #[error("token not found")]
    TokenNotFound,

    #[error("invalid token: {message}")]
    InvalidToken { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub status: u16,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::Unknown { message } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse {
                    code: "E_INTERNAL_SERVER_ERROR".to_string(),
                    status: 500,
                    message: format!("internal server error: {message}"),
                }),
            )
                .into_response(),

            ApiError::TokenNotFound => (
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse {
                    code: "E_UNAUTHORIZED".to_string(),
                    status: 401,
                    message: "token not found".to_string(),
                }),
            )
                .into_response(),

            ApiError::InvalidToken { message } => (
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse {
                    code: "E_UNAUTHORIZED".to_string(),
                    status: 401,
                    message,
                }),
            )
                .into_response(),
        }
    }
}
