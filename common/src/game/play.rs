//! Module representing a [`Play`] (move) made by a player.

use super::{
    pos::{Direction, Pos},
    tile::{Letter, Tile},
};
use std::{fmt, iter};

/// A Play is the chosen action by a player on their turn. Each
/// play can either be a [`Pass`](Play::Pass), redrawing of some
/// of the tiles in the player's rack [`RedrawTiles`](Play::RedrawTiles),
/// or the placement of up to 7 tiles on the board.
///
/// Plays are not validated until they are played on a board, so
/// [`Play`] simply allows information to be stored for later use.
pub enum Play {
    /// The turn is forfeit
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
                    write!(f, "({},{})", pos, tile.to_string().trim())?;
                }

                write!(f, ")")
            }
        }
    }
}
impl Play {
    /// Creates a [`Pass`](Play::Pass).
    pub fn pass() -> Self {
        Self::Pass
    }
    /// Creates a [`Redraw`](Play::Redraw) play from an iterator
    /// over tiles.
    pub fn redraw<I>(tiles: I) -> Self
    where
        I: Iterator<Item = Tile>,
    {
        Self::Redraw(tiles.collect())
    }
    /// Creates a [`Place`](Play::Place) play from an iterator
    /// over tiles and positions.
    pub fn place<I>(tiles: I) -> Self
    where
        I: Iterator<Item = (Pos, Tile)>,
    {
        Self::Place(tiles.collect())
    }
    /// Creates a [`Place`](Play::Place) play from an iterator
    /// over [words](Word).
    pub fn place_words<I>(words: I) -> Self
    where
        I: Iterator<Item = Word>,
    {
        Self::Place(words.flat_map(|w| w.into_iter()).collect())
    }
}

/// A [`Word`] provides a simpler way to construct a [`PlaceTiles`](Play::PlaceTiles)
/// play. A string, start position, and direction is given, then the required
/// tiles can be determined.
#[derive(Debug, Clone)]
pub struct Word {
    tile_positions: Vec<(Pos, Tile)>,
}

impl Word {
    /// Creates a new [`Word`] from a string, direction and start position.
    /// Returns `None` if the word would run off the board. Does not validate
    /// the word itself.
    pub fn new<T>(word: &str, dir: Direction, start: T) -> Option<Self>
    where
        T: Into<Pos>,
    {
        let start: Pos = start.into();
        // Filter out anything that is not a letter.
        let letters: Vec<_> = word.chars().filter_map(Letter::new).collect();

        start.offset(dir, letters.len()).map(|_| Self {
            // convert each letter to a tile and combine with its board position.
            tile_positions: iter::successors(Some(start), |pos| pos.offset(dir, 1))
                .zip(letters.into_iter().map(Tile::from))
                .collect(),
        })
    }
    /// Changes the tile at `pos` to be a blank. Return value indicates success.
    /// This may fail if the letter at `pos` has already been set to a blank,
    /// or if there is no letter at `pos`.
    pub fn use_blank_at<T>(&mut self, pos: T) -> bool
    where
        T: Into<Pos>,
    {
        let pos = pos.into();

        if let Some((_, tile @ Tile::Letter(_))) =
            self.tile_positions.iter_mut().find(|(p, _)| *p == pos)
        {
            *tile = Tile::Blank(tile.letter());
            true
        } else {
            false
        }
    }
    /// Same as `use_blank_at`, but provide the offset in the word
    /// rather than the actual position of the tile to change to a
    /// blank.
    pub fn use_blank_at_offset(&mut self, offset: usize) -> bool {
        match self.tile_positions.get(offset).copied() {
            None => false,
            Some((pos, _)) => self.use_blank_at(pos),
        }
    }
}
impl IntoIterator for Word {
    type Item = (Pos, Tile);
    type IntoIter = std::vec::IntoIter<(Pos, Tile)>;

    fn into_iter(self) -> Self::IntoIter {
        self.tile_positions.into_iter()
    }
}
impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (_, t) in &self.tile_positions {
            write!(f, "{}", t.to_string().trim())?;
        }

        Ok(())
    }
}
