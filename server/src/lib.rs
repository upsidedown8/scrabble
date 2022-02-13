use scrabble::game::word_tree::WordTree;
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

    warp::serve(routes.with(cors)).run(addr).await;
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

/// Loads the word tree from the $WORD_LIST directory.
async fn load_word_tree() -> tokio::io::Result<WordTree> {
    let word_file = env::var("WORD_LIST").expect("`WORD_LIST` env variable");

    log::info!("building word tree from file: {}", word_file);

    let mut word_tree = WordTree::default();
    let file = File::open(word_file)
        .await
        .expect("word file should exist at `WORD_LIST` dir");
    let mut lines = BufReader::new(file).lines();

    while let Some(line) = lines.next_line().await? {
        word_tree.insert(line.trim());
    }

    Ok(word_tree)
}
