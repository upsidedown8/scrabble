#[macro_use]
extern crate rocket;

use std::env;

use anyhow::Result;

use rocket::{http::Status, State};

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

#[derive(Debug)]
pub struct User {
    pub id_user: i64,
    pub username: String,
    pub password: String,
    pub salt: String,
}

impl User {
    pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<Self> {
        let user = sqlx::query_as!(User, "SELECT * FROM tbl_user WHERE id_user = ?", id)
            .fetch_one(&*pool)
            .await?;

        Ok(user)
    }
}

#[get("/user/<id>")]
async fn user(pool: &State<SqlitePool>, id: i32) -> Result<String, Status> {
    let user = User::find_by_id(id, pool).await;

    match user {
        Ok(user) => Ok(format!("Hi {}", user.username)),
        _ => Err(Status::NotFound),
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("A `.env` file to be present in the working directory");

    let db_url = env::var("DATABASE_URL")?;

    println!("url: {}", db_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    rocket::build()
        .mount("/", routes![user])
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}
