mod api;
mod common;
mod db;
mod middlewares;
mod oauth2;
mod routes;

use axum::{routing::IntoMakeService, Router};
use hyper::{server::conn::AddrIncoming, Server};
use std::net::TcpListener;

pub use self::common::*;
pub type AppServer = Server<AddrIncoming, IntoMakeService<Router>>;

pub async fn run(listener: TcpListener) -> Result<AppServer, hyper::Error> {
    let state = AppState::new().await;

    let app = routes::routes(&state).with_state(state);
    let app = middlewares::with_middleware_stack(app);

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
