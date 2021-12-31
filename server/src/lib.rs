#[doc = include_str!("../../README.md")]
pub mod auth;
pub mod db;
pub mod routes;

#[macro_use]
extern crate rocket;

use argon2::{Config, ThreadMode};
use common::game::word_tree::WordTree;
use rocket::{
    tokio::{
        fs::File,
        io::{AsyncBufReadExt, BufReader},
    },
    Build, Rocket,
};
use routes::{users, words};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;

pub struct AppState<'a> {
    pub pool: SqlitePool,
    pub word_tree: WordTree,
    pub hash_cfg: Config<'a>,
}

pub async fn build_rocket() -> anyhow::Result<Rocket<Build>> {
    dotenv::dotenv().expect("`.env` should be present in working directory");

    log::info!("loading env vars");
    let db_url = env::var("DATABASE_URL").expect("expected `DATABASE_URL` environment variable");
    let word_file = env::var("WORDLIST").expect("expected `WORDLIST` environment variable");
    // jsut to check it exists, as `JWT_SECRET` is a lzy static
    let _ = env::var("JWT_SECRET").expect("expected `JWT_SECRET` environment variable");

    log::info!("connecting to database: {}", db_url);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    log::info!("building word tree from file: {}", word_file);
    let mut word_tree = WordTree::default();
    let file = File::open(word_file)
        .await
        .expect("word file should exist at `WORDLIST` dir");
    let mut lines = BufReader::new(file).lines();

    while let Some(line) = lines.next_line().await? {
        word_tree.insert(line.trim());
    }

    let hash_cfg = Config {
        thread_mode: ThreadMode::Parallel,
        ..Config::default()
    };

    Ok(Rocket::build()
        .mount(
            "/api",
            routes![
                users::create,
                users::login,
                users::update,
                users::delete,
                users::get_details,
                words::check,
            ],
        )
        .manage(AppState {
            pool,
            word_tree,
            hash_cfg,
        }))
}
