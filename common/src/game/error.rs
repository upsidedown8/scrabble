use std::{error::Error, fmt};

/// The [`Result`] type for the `game` module.
pub type GameResult<T> = std::result::Result<T, GameError>;

/// The error type for the game module.
#[derive(Debug)]
pub enum GameError {
    /// A placed word was not in the word list.
    InvalidWord,
    /// Expected at least one tile to place.
    ZeroTilesPlaced,
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
    /// At most 7 tiles can be placed during a play.
    MaximumTilesExceeded,
}

impl Error for GameError {}
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "todo")
    }
}
