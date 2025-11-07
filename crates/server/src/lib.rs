use std::net::{SocketAddr, ToSocketAddrs};

use axum::Router;
use tracing::{error, info};

pub mod args;
pub mod http;

pub async fn get_addr(host: &str, port: u16) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    let addrs = format!("{}:{}", host, port)
        .to_socket_addrs()?
        .collect::<Vec<SocketAddr>>();

    let socket = match addrs.first() {
        Some(addr) => *addr,
        None => return Err("Could not resolve address".into()),
    };

    Ok(socket)
}

pub async fn run_server(addr: SocketAddr, router: Router) {
    info!("listening on {addr}");

    if let Err(e) = axum_server::bind(addr)
        .serve(router.clone().into_make_service())
        .await
    {
        error!("server error: {}", e);
        std::process::exit(1);
    }
}
