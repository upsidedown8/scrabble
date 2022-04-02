//! The server library: provides the REST API and live game
//! protocol over WebSockets to manage state for the app. Data
//! is stored in a PostgreSQL database.

// Produce a compiler warning for missing documentation.
#![warn(missing_docs)]

use db::Db;
use error::Result;
use fsm::FsmHandle;
use mailer::Mailer;
use std::{env, net::SocketAddr};
use warp::{http::Method, Filter};

mod auth;
mod db;
mod error;
mod filters;
mod fsm;
mod handlers;
mod mailer;
mod models;

/// The main entry point for the server.
#[tokio::main]
async fn main() -> Result<()> {
    // initialize the logger.
    env_logger::init();
    // load `.env` file.
    dotenv::dotenv().expect("`.env` file to be present");

    // load and parse the socket address.
    let socket_addr = env::var("SOCKET_ADDRESS")?;
    let addr = socket_addr.parse()?;

    // run the server.
    serve(addr).await?;

    Ok(())
}

/// Starts the server on the given address.
async fn serve(addr: SocketAddr) -> Result<()> {
    // load TLS certificate and private key.
    let cert_path = env::var("CERT_PATH")?;
    let key_path = env::var("KEY_PATH")?;

    // load allowed origin.
    let origin = env::var("ORIGIN")?;

    // set up database connection, mail connection, and load the fsm.
    let db = db::connect().await?;
    let mailer = Mailer::new_from_env()?;
    let fsm = FsmHandle::new_from_env()?;

    // CORS settings, which set allowed origins, headers and methods.
    let cors = warp::cors()
        .allow_origin(origin.as_str())
        .allow_methods(&[
            Method::GET,
            Method::OPTIONS,
            Method::POST,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_headers(vec!["authorization", "content-type"]);

    // specify the hostname.
    let hostname = env::var("HOSTNAME")?;

    // serve on `addr`.
    warp::serve(filters::all(&hostname, db, mailer, fsm).with(cors))
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(addr)
        .await;

    Ok(())
}
