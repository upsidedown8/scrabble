//! Module modelling the scrabble tile.

use crate::error::{GameError, GameResult};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display, Formatter};

/// A letter `A..=Z`. Represented as a newtype containing an unsigned
/// integer from `0..=25` to make game operations easier.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Letter(#[serde(deserialize_with = "deserialize_letter")] u8);

/// Custom deserializer that ensures that deserialized letter values
/// are valid.
fn deserialize_letter<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        // letters are only valid for (0..=25).
        byte @ 0..=25 => Ok(byte),
        _ => Err(serde::de::Error::custom("Byte out of letter range")),
    }
}

impl Letter {
    /// Creates a new letter from a `char`, returns [`None`] if the
    /// `ch` provided is not in the latin alphabet.
    pub fn new(ch: char) -> Option<Self> {
        match ch {
            'a'..='z' => Some(Letter(ch as u8 - 97)),
            'A'..='Z' => Some(Letter(ch as u8 - 65)),
            _ => None,
        }
    }
    /// Returns an iterator over all 26 letters
    pub fn iter() -> impl Iterator<Item = Letter> {
        (0..26).map(Letter::from)
    }
}
impl From<usize> for Letter {
    fn from(v: usize) -> Self {
        Self((v as u8) % 26)
    }
}
impl From<Letter> for usize {
    fn from(letter: Letter) -> Self {
        letter.0 as usize
    }
}
impl From<Letter> for char {
    fn from(letter: Letter) -> Self {
        (letter.0 + 65) as char
    }
}
impl Display for Letter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
impl fmt::Debug for Letter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", char::from(*self))
    }
}

/// A scrabble tile, one of the 26 letters or a blank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Tile {
    /// The tile is a letter (A..=Z)
    Letter(Letter),
    /// The tile is a blank, which can act as any single letter,
    /// but has zero score.
    Blank(Option<Letter>),
}

impl From<Option<Letter>> for Tile {
    fn from(op: Option<Letter>) -> Self {
        Self::Blank(op)
    }
}
impl From<Letter> for Tile {
    fn from(letter: Letter) -> Self {
        Self::Letter(letter)
    }
}
impl From<Tile> for usize {
    fn from(tile: Tile) -> Self {
        match tile {
            // a letter is from `0..=25`
            Tile::Letter(Letter(num)) => num as usize,
            // a blank is `26`
            Tile::Blank(_) => 26,
        }
    }
}
impl From<usize> for Tile {
    fn from(tile: usize) -> Self {
        match tile {
            0..=25 => Tile::Letter(Letter::from(tile)),
            _ => Tile::Blank(None),
        }
    }
}
impl From<char> for Tile {
    fn from(ch: char) -> Tile {
        match Letter::new(ch) {
            Some(letter) => Tile::Letter(letter),
            None => Tile::Blank(None),
        }
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Letter(l) => write!(f, " {} ", l),
            Tile::Blank(Some(l)) => write!(f, "({})", l),
            Tile::Blank(_) => write!(f, "( )"),
        }
    }
}
impl Tile {
    /// Checks whether `self` is a blank tile.
    pub fn is_blank(&self) -> bool {
        matches!(self, Tile::Blank(_))
    }
    /// A blank tile.
    pub fn blank() -> Tile {
        Tile::Blank(None)
    }
    /// Gets the optional letter that the tile represents. If the letter
    /// is not present (not specified for a blank), returns an error.
    pub fn letter(&self) -> GameResult<Letter> {
        match self {
            Tile::Letter(l) => Ok(*l),
            Tile::Blank(opt) => opt.ok_or(GameError::MissingLetter),
        }
    }
    /// Returns an iterator over all 27 tiles.
    pub fn iter() -> impl Iterator<Item = Tile> {
        (0..27).map(Tile::from)
    }
    /// Gets the score of the tile
    pub fn score(&self) -> usize {
        const TILE_SCORES: [usize; 27] = [
            1,  // A
            3,  // B
            3,  // C
            2,  // D
            1,  // E
            4,  // F
            2,  // G
            4,  // H
            1,  // I
            8,  // J
            5,  // K
            1,  // L
            3,  // M
            1,  // N
            1,  // O
            3,  // P
            10, // Q
            1,  // R
            1,  // S
            1,  // T
            1,  // U
            4,  // V
            4,  // W
            8,  // X
            4,  // Y
            10, // Z
            0,  // Blank
        ];

        TILE_SCORES[usize::from(*self)]
    }
}
