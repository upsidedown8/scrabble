use scrabble::util::fsm::{Fsm, FsmBuilder};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;
use std::net::SocketAddr;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};
use warp::{http::Method, Filter};

mod auth;
mod error;
mod models;
mod routes;

/// Starts the server on the given address.
pub async fn serve(addr: impl Into<SocketAddr>) {
    let cert_path = env::var("CERT_PATH").expect("`CERT_PATH` env variable to be set");
    let key_path = env::var("KEY_PATH").expect("`KEY_PATH` env variable to be set");

    let db = connect_db().await.unwrap();
    let routes = routes::all(db);
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

    log::info!("connecting to database: {}", db_url);
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}

/// Loads the fsm from the $WORD_LIST directory.
async fn load_fsm_from_wordlist<'a, F: Fsm<'a>>() -> tokio::io::Result<F> {
    let word_file = env::var("WORD_LIST").expect("`WORD_LIST` env variable");

    log::info!("building fsm from file: {}", word_file);

    let mut fsm_builder = FsmBuilder::default();
    let file = File::open(word_file)
        .await
        .expect("word file should exist at `WORD_LIST` dir");
    let mut lines = BufReader::new(file).lines();

    while let Some(line) = lines.next_line().await? {
        fsm_builder.insert(line.trim());
    }

    Ok(fsm_builder.build())
}
