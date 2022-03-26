use scrabble::util::fsm::FastFsm;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::net::SocketAddr;
use std::{env, sync::Arc};
use warp::{http::Method, Filter};

mod auth;
mod error;
mod models;
mod routes;

/// Starts the server on the given address.
pub async fn serve(addr: impl Into<SocketAddr>) {
    let cert_path = env::var("CERT_PATH").expect("`CERT_PATH` env variable to be set");
    let key_path = env::var("KEY_PATH").expect("`KEY_PATH` env variable to be set");

    // load the finite state machine and store in an atomic reference counter.
    let fsm = load_fast_fsm().expect("load `FastFsm` from binary file");
    let fsm = Arc::new(fsm);

    // connect to the database.
    let db = connect_db().await.expect("database connection");

    // the api endpoints.
    let routes = routes::all(&db, &fsm);
    // CORS settings, which set allowed origins, headers and methods.
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[
            Method::GET,
            Method::OPTIONS,
            Method::POST,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_headers(vec!["authorization", "content-type"]);

    // serve on `addr`.
    warp::serve(routes.with(cors))
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(addr)
        .await;
}

/// Connects to the database at $DATABASE_URL.
async fn connect_db() -> sqlx::Result<SqlitePool> {
    let db_url = env::var("DATABASE_URL").expect("`DATABASE_URL` env variable");

    log::info!("connecting to database: {db_url}");
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

/// Loads the `FastFsm` from the binary file.
fn load_fast_fsm() -> Result<FastFsm, Box<dyn std::error::Error>> {
    let fsm_path = env::var("FAST_FSM_BIN").expect("`FAST_FSM_BIN` env variable");

    log::info!("loading fast fsm: {fsm_path}");
    let file = std::fs::File::open(&fsm_path)?;
    let rdr = std::io::BufReader::new(file);
    let fast_fsm: FastFsm = bincode::deserialize_from(rdr)?;

    Ok(fast_fsm)
}
