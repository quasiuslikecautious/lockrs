mod common;

pub mod api;
pub mod db;
pub mod oauth2;

mod middlewares;
mod routes;

use axum::{routing::IntoMakeService, Router};
use hyper::{server::conn::AddrIncoming, Server};
use std::{net::TcpListener, sync::Arc};

pub use self::common::*;
pub type AppServer = Server<AddrIncoming, IntoMakeService<Router>>;

pub async fn run(
    listener: TcpListener,
    state: Option<AppState>,
) -> Result<AppServer, hyper::Error> {
    let state = state.unwrap_or(AppState::new(None).await);

    let app = routes::routes(&state).with_state(state);
    let app = middlewares::with_middleware_stack(app);

    let server = axum::Server::from_tcp(listener)?.serve(app.into_make_service());

    Ok(server)
}
