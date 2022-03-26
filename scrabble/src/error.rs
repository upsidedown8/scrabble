//! The error and result types for the library.

use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

/// The [`Result`] type for the [`game`](super::game) module.
pub type GameResult<T> = std::result::Result<T, GameError>;

/// The error type for the game module.
#[derive(Debug, Serialize, Deserialize)]
pub enum GameError {
    /// The letter bag does not contain enough letters to redraw the requested tiles.
    NotEnoughLetters,
    /// Attempted to play a tile which was not in the player's rack.
    NotInRack,
    /// Cannot make a play as the game is over.
    Over,
    /// A placed word was not in the word list.
    InvalidWord,
    /// Expected at least one and no more than 7 tiles to place.
    PlacementCount,
    /// The tiles added during a play would have overlayed the existing tiles.
    CoincedentTiles,
    /// At least one pair of tiles added during a play were placed on the same square.
    DuplicatePosition,
    /// At least one and no more than 7 tiles may be redrawn from the bag.
    RedrawCount,
    /// There must be a tile on the start square.
    MustIntersectStart,
    /// Every word needs at least two letters.
    WordsNeedTwoLetters,
    /// Every tile should have a neighbour above, below, left or right.
    NotConnected,
    /// Placed tiles must share a common row or column.
    NoCommonLine,
    /// A blank tile placed on the board did not specify a letter.
    MissingLetter,
}

impl Error for GameError {}
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameError::NotEnoughLetters => "There are not enough letters in the bag to redraw",
                GameError::NotInRack => "One or more placed tiles were not in the rack",
                GameError::Over => "The game is over so no futher plays can be made",
                GameError::InvalidWord => "A word was not in the dictionary",
                GameError::PlacementCount => "At least 1 and no more than 7 tiles can be placed",
                GameError::CoincedentTiles => "Tiles were placed over existing tiles",
                GameError::DuplicatePosition => "Multiple tiles were placed on the same square",
                GameError::RedrawCount =>
                    "At least 1 and up to the number of tiles on the rack can be redrawn",
                GameError::MustIntersectStart => "A tile must be placed on the start square",
                GameError::WordsNeedTwoLetters => "Words need at least 2 letters",
                GameError::NotConnected => "Not connected",
                GameError::NoCommonLine => "Placed tiles must share a common row or column",
                GameError::MissingLetter =>
                    "A blank tile placed on the board did not specify a letter",
            }
        )
    }
}
