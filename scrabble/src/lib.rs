//! Implementation of a multiplayer scrabble game and AI.

// show a compiler warning when public api types are not documented.
#![warn(missing_docs)]

pub mod ai;
pub mod bitboard;
pub mod board;
pub mod error;
pub mod game;
pub mod letter_bag;
pub mod play;
pub mod pos;
pub mod rack;
pub mod tile;
pub mod word_tree;
