#[doc = include_str!("../../README.md")]
pub mod auth;
pub mod db;
pub mod routes;

#[macro_use]
extern crate rocket;

use argon2::{Config, ThreadMode};
use common::game::word_tree::WordTree;
use rand::Rng;
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

pub struct JwtSettings {
    pub jwt_secret: Vec<u8>,
    pub jwt_expiry_delta: usize,
    pub jwt_session: usize,
}

pub struct AppState<'a> {
    pub pool: SqlitePool,
    pub word_tree: WordTree,
    pub hash_cfg: Config<'a>,
    pub jwt: JwtSettings,
}

pub async fn build_rocket() -> anyhow::Result<Rocket<Build>> {
    dotenv::dotenv().expect("`.env` should be present in working directory");

    log::info!("loading env vars");
    let db_url = env::var("DATABASE_URL").expect("expected `DATABASE_URL` environment variable");
    let word_file = env::var("WORDLIST").expect("expected `WORDLIST` environment variable");
    let jwt_secret = env::var("JWT_SECRET").expect("expected `JWT_SECRET` environment variable");
    let jwt_secret = hex::decode(&jwt_secret).expect("`JWT_SECRET` should be valid hex");
    let jwt_expiry_delta: usize = env::var("JWT_EXPIRY")
        .expect("expected `JWT_EXPIRY` environment variable")
        .parse()
        .expect("`JWT_EXPIRY` should be a positive integer");
    // generate a random number so that the jwt is only
    // valid for the current server session
    let jwt_session: usize = rand::thread_rng().gen();

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
            jwt: JwtSettings {
                jwt_secret,
                jwt_expiry_delta,
                jwt_session,
            },
        }))
}
