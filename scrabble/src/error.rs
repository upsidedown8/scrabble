use std::{error::Error, fmt};

/// The [`Result`] type for the `game` module.
pub type GameResult<T> = std::result::Result<T, GameError>;

/// The error type for the game module.
#[derive(Debug)]
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
}

impl Error for GameError {}
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "todo")
    }
}
