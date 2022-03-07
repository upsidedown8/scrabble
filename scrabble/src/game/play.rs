//! Module representing a [`Play`] (move) made by a player.

use crate::{
    game::{
        board::Board,
        tile::{Letter, Tile},
    },
    util::pos::{Direction, Pos},
};
use serde::{Deserialize, Serialize};
use std::{fmt, iter};

/// A Play is the chosen action by a player on their turn. Each
/// play can either be a [`Pass`](Play::Pass), redrawing of some
/// of the tiles in the player's rack [`Redraw`](Play::Redraw),
/// or the placement of up to 7 tiles on the board.
///
/// Plays are not validated until they are played on a board, so
/// [`Play`] simply allows information to be stored for later use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Play {
    /// The turn is forfeit.
    Pass,
    /// Up to 7 tiles in the player's rack are redrawn from
    /// the bag.
    Redraw(Vec<Tile>),
    /// The player places up to 7 tiles on the board.
    Place(Vec<(Pos, Tile)>),
}
impl fmt::Display for Play {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Play::Pass => write!(f, "Pass"),
            Play::Redraw(tiles) => {
                write!(f, "Redraw(",)?;

                for t in tiles {
                    write!(f, "{},", t.to_string().trim())?;
                }

                write!(f, ")")
            }
            Play::Place(tile_positions) => {
                write!(f, "Place(")?;

                for (pos, tile) in tile_positions {
                    write!(f, "({},{}),", pos, tile.to_string().trim())?;
                }

                write!(f, ")")
            }
        }
    }
}
impl Play {
    /// Place a horizontal word.
    pub fn horizontal(start: impl Into<Pos>) -> PlaceBuilder {
        PlaceBuilder::horizontal(start)
    }
    /// Place a vertical word.
    pub fn vertical(start: impl Into<Pos>) -> PlaceBuilder {
        PlaceBuilder::vertical(start)
    }
    /// Redraw some tiles.
    pub fn redraw() -> RedrawBuilder {
        RedrawBuilder::default()
    }
    /// Pass the move.
    pub fn pass() -> Self {
        Self::Pass
    }
}

/// Convenient way to create a [`Play::Redraw`].
#[derive(Debug, Default)]
pub struct RedrawBuilder {
    tiles: Vec<Tile>,
}
impl RedrawBuilder {
    /// Adds a letter tile to the list to redraw.
    pub fn letter(mut self, letter: impl Into<Letter>) -> Self {
        self.tiles.push(Tile::Letter(letter.into()));
        self
    }
    /// Adds a blank tile to the list to redraw.
    pub fn blank(mut self) -> Self {
        self.tiles.push(Tile::Blank(None));
        self
    }
    /// Constructs a [`Play::Redraw`] from the tiles.
    pub fn build(self) -> Play {
        Play::Redraw(self.tiles)
    }
}

/// Convenient way to create a [`Play::Place`].
#[derive(Debug)]
pub struct PlaceBuilder {
    start: Pos,
    dir: Direction,
    tiles: Vec<Tile>,
}
impl PlaceBuilder {
    /// A builder for a horizontal word.
    pub fn horizontal(start: impl Into<Pos>) -> Self {
        Self {
            start: start.into(),
            dir: Direction::East,
            tiles: vec![],
        }
    }
    /// A builder for vertical word.
    pub fn vertical(start: impl Into<Pos>) -> Self {
        Self {
            start: start.into(),
            dir: Direction::South,
            tiles: vec![],
        }
    }
    /// Adds multiple letters to the current word.
    pub fn letters(mut self, letters: &str) -> Self {
        self.tiles
            .extend(letters.chars().filter_map(Letter::new).map(Tile::Letter));
        self
    }
    /// Adds a letter tile to the current word.
    pub fn letter(mut self, letter: impl Into<Letter>) -> Self {
        self.tiles.push(Tile::Letter(letter.into()));
        self
    }
    /// Adds a blank tile to the current word.
    pub fn blank(mut self, letter: impl Into<Letter>) -> Self {
        self.tiles.push(Tile::Blank(Some(letter.into())));
        self
    }
    /// Returns a play containing only the tiles that would not interfere with
    /// existing ones on the board.
    pub fn build(self, board: &Board) -> Play {
        Play::Place(self.tile_positions(board))
    }
    /// Consumes the builder, producing a vec of tile positions.
    pub fn tile_positions(self, board: &Board) -> Vec<(Pos, Tile)> {
        iter::successors(Some(self.start), |x| x.offset(self.dir, 1))
            .zip(self.tiles)
            .filter(|&(pos, _)| board.at(pos).is_none())
            .collect()
    }
}
