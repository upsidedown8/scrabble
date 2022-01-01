//! Module modelling the scrabble tile.

use std::fmt::{Display, Formatter};

use super::letter_bag::TILE_COUNT;

/// A letter `A..=Z`. Represented as a newtype containing an unsigned
/// integer from `0..=25` to make game operations easier.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Letter(usize);

impl Letter {
    /// Creates a new letter from a `char`, returns [`None`] if the
    /// `ch` provided is not in the latin alphabet.
    pub fn new(ch: char) -> Option<Self> {
        match ch {
            'a'..='z' => Some(Letter(ch as usize - 97)),
            'A'..='Z' => Some(Letter(ch as usize - 65)),
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
        Self(v % 26)
    }
}
impl From<Letter> for usize {
    fn from(letter: Letter) -> Self {
        letter.0
    }
}
impl Display for Letter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.0 + 65) as u8 as char)
    }
}

/// A scrabble tile, one of the 26 letters or a blank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
            Tile::Letter(Letter(num)) => num,
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
    /// Gets the optional letter that the tile represents.
    pub fn letter(&self) -> Option<Letter> {
        match self {
            Tile::Letter(l) => Some(*l),
            Tile::Blank(opt) => *opt,
        }
    }
    /// Returns an iterator over all 27 tiles.
    pub fn iter() -> impl Iterator<Item = Tile> {
        (0..TILE_COUNT).map(Tile::from)
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
