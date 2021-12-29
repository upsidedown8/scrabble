//! Module for handling game logic.

use std::{
    io::{self, BufRead, BufReader},
    path::Path,
};

use {board::Board, rack::Rack, word_tree::WordTree};

pub mod bitboard;
pub mod board;
pub mod play;
pub mod rack;
pub mod tile;
pub mod word_tree;

pub struct Player(String);
pub struct Score(usize);

pub enum GameStatus {
    Winner(Player, Score),
    ToPlay(Player),
}

pub struct Game {
    tree: WordTree,
    racks: Vec<Rack>,
    board: Board,
}

impl Game {
    pub fn new<P>(word_file: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        // read words from file and populate tree
        let mut tree = WordTree::default();

        let file = std::fs::File::open(word_file)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            tree.insert(line?.trim());
        }

        Ok(Self {
            tree,
            board: Board::default(),
            racks: vec![],
        })
    }
}
