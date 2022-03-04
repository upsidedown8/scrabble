//! Module representing a [`Play`] (move) made by a player.

use crate::{game::tile::Tile, util::pos::Pos};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A Play is the chosen action by a player on their turn. Each
/// play can either be a [`Pass`](Play::Pass), redrawing of some
/// of the tiles in the player's rack [`Redraw`](Play::Redraw),
/// or the placement of up to 7 tiles on the board.
///
/// Plays are not validated until they are played on a board, so
/// [`Play`] simply allows information to be stored for later use.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
                    write!(f, "({},{}),", pos, tile.to_string().trim())?;
                }

                write!(f, ")")
            }
        }
    }
}
