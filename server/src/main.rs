use std::io::{BufRead, BufReader};

use server::game::{
    board::{Board, Direction, Pos},
    play::{Play, Word},
    tile::{Letter, Tile},
    word_tree::*,
};

// #[macro_use]
// extern crate rocket;

// use std::env;

// use anyhow::Result;

// use rocket::{http::Status, State};

// use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

// #[derive(Debug)]
// pub struct User {
//     pub id_user: i64,
//     pub username: String,
//     pub password: String,
//     pub salt: String,
// }

// impl User {
//     pub async fn find_by_id(id: i32, pool: &SqlitePool) -> Result<Self> {
//         let user = sqlx::query_as!(User, "SELECT * FROM tbl_user WHERE id_user = ?", id)
//             .fetch_one(&*pool)
//             .await?;

//         Ok(user)
//     }
// }

// #[get("/user/<id>")]
// async fn user(pool: &State<SqlitePool>, id: i32) -> Result<String, Status> {
//     let user = User::find_by_id(id, pool).await;

//     match user {
//         Ok(user) => Ok(format!("Hi {}", user.username)),
//         _ => Err(Status::NotFound),
//     }
// }

// #[rocket::main]
// async fn main() -> Result<()> {
// dotenv::dotenv().expect("A `.env` file to be present in the working directory");

// let db_url = env::var("DATABASE_URL")?;

// println!("url: {}", db_url);

// let pool = SqlitePoolOptions::new()
//     .max_connections(5)
//     .connect(&db_url)
//     .await?;

// rocket::build()
//     .mount("/", routes![user])
//     .manage(pool)
//     .launch()
//     .await?;

//     Ok(())
// }

fn main() {
    let mut board = Board::default();

    let w = vec![
        Word::new("apple", Direction::Down, (5, 3)).unwrap(),
        Word::new("please", Direction::Right, (6, 2)).unwrap(),
    ];
    let play = Play::from_words(w.into_iter());

    println!("{}", play.occupancy());

    board.make_play(play);

    let t = vec![
        (Pos::from((10, 10)), Tile::Letter(Letter::new('a').unwrap())),
        (Pos::from((11, 11)), Tile::Blank(Letter::new('a'))),
        (Pos::from((12, 12)), Tile::Blank(None)),
    ];

    board.make_play(Play::from_tiles(t.into_iter()));

    println!("{}", board);

    // let mut tree = WordTree::default();

    // let file = std::fs::File::open("res/words.txt").unwrap();
    // let reader = BufReader::new(file);

    // for line in reader.lines() {
    //     tree.insert(line.unwrap().trim());
    // }

    // for &child in tree.node(tree.root_idx()).children() {
    //     println!("{:?}", child);
    // }

    // let mut buf = String::new();
    // let stdin = std::io::stdin();
    // let mut stdin = stdin.lock();

    // loop {
    //     buf.clear();
    //     stdin.read_line(&mut buf).unwrap();

    //     println!("{}", tree.contains(buf.trim()));
    // }
}
