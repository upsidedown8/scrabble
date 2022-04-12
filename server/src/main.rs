//! The server library: provides the REST API and live game
//! protocol over WebSockets to manage state for the app. Data
//! is stored in a PostgreSQL database.

// Produce a compiler warning for missing documentation.
#![warn(missing_docs)]

use db::Db;
use error::Result;
use fsm::FsmHandle;
use mailer::Mailer;
use std::env;
use warp::{cors::Cors, http::Method, Filter};

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

    // run a HTTP redirect server.
    let http = tokio::spawn(serve_http());
    // run an HTTPS server.
    let https = tokio::spawn(serve_https());

    let result;
    // wait for either of the servers to run in to an error.
    tokio::select! {
        r = http => result = r.unwrap(),
        r = https => result = r.unwrap(),
    };

    result
}

/// Builds the CORS request wrapper.
fn cors(is_https: bool) -> Result<Cors> {
    // load the domain name.
    let domain = env::var("DOMAIN")?;

    // Only allow origins from the app fqdn.
    let origin = if is_https {
        format!("https://{domain}")
    } else {
        format!("http://{domain}")
    };

    // CORS settings, which set allowed origins, headers and methods.
    let cors = warp::cors()
        // .allow_origin(origin.as_str())
        .allow_any_origin()
        .allow_methods(&[
            Method::GET,
            Method::OPTIONS,
            Method::POST,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_headers(vec!["authorization", "content-type"])
        .build();

    Ok(cors)
}

/// Starts a server that redirects requests from :80 to :443.
async fn serve_http() -> Result<()> {
    let routes = filters::http_redirect().recover(filters::handle_rejection);
    let cors = cors(false)?;

    warp::serve(routes.with(cors)).run(([0, 0, 0, 0], 80)).await;

    Ok(())
}

/// Starts a HTTPS server on localhost:443.
async fn serve_https() -> Result<()> {
    // load TLS certificate and private key.
    let cert_path = env::var("CERT_PATH")?;
    let key_path = env::var("KEY_PATH")?;

    // set up database connection, mail connection, and load the fsm.
    let db = db::connect().await?;
    let mailer = Mailer::new_from_env()?;
    let fsm = FsmHandle::new_from_env()?;

    // handlers for the endpoints.
    let routes = filters::all(db, mailer, fsm)?.recover(filters::handle_rejection);
    let cors = cors(true)?;

    // serve on localhost:443.
    warp::serve(routes.with(cors))
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(([0, 0, 0, 0], 443))
        .await;

    Ok(())
}
