//! A database pool that can be shared across threads.

use crate::error::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

/// Thread safe database pool.
pub type Db = PgPool;

/// Connects to the database url in $DATABASE_URL.
pub async fn connect() -> Result<Db> {
    let db_url = env::var("DATABASE_URL")?;

    log::info!("connecting to database: {db_url}");
    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    Ok(pg_pool)
}
