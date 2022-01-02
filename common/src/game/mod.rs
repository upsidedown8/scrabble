//! Module for handling abstract game representation and player
//! interaction (uncoupled from DB, UI and API).
//!
//! Game logic is shared between the client and server, so that
//! API calls can be minimised, (only for actually making moves)
//! by performing the majority of validation on the client side.
//! The types exposed in this module are also useful for modelling
//! state for the UI.

use self::{
    board::Board,
    error::{GameError, GameResult},
    letter_bag::LetterBag,
    play::Play,
    rack::Rack,
    word_tree::WordTree,
};

pub mod bitboard;
pub mod board;
pub mod error;
pub mod letter_bag;
pub mod play;
pub mod pos;
pub mod rack;
pub mod tile;
pub mod word_tree;

/// The reason that the game has ended.
#[derive(Clone, Debug)]
pub enum Reason {
    /// A player has emptied their rack with no letters remaining in the bag.
    EmptyRack,
    /// All players have passed for 6 consecutive rounds.
    SixPasses,
}

/// The current state of the game.
#[derive(Clone, Debug)]
pub enum GameStatus {
    /// One or more players have one
    Over(Vec<PlayerId>, Reason),
    /// The game is ongoing.
    ToPlay(PlayerId),
}

/// Used to identify a player.
#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerId(usize);
impl From<PlayerId> for usize {
    fn from(PlayerId(id): PlayerId) -> Self {
        id
    }
}

/// Top level struct allowing for management of the entire
/// game. Manages players, all state, and determines when the
/// game is over, calculating scores and determining the winner.
pub struct Game<'a> {
    word_tree: &'a WordTree,

    board: Board,
    letter_bag: LetterBag,
    scores: Vec<usize>,
    racks: Vec<Rack>,

    to_play: PlayerId,
    pass_count: usize,
    player_count: usize,
    status: GameStatus,
}

impl<'a> Game<'a> {
    /// Constructs a new [`Game`] from a borrowed `word_tree` and the number
    /// of players.
    pub fn new(word_tree: &'a WordTree, player_count: usize) -> Self {
        let mut letter_bag = LetterBag::default();

        let to_play = PlayerId(0);
        let racks = (0..player_count)
            .map(|_| Rack::new(&mut letter_bag))
            .collect();

        Self {
            word_tree,
            letter_bag,
            to_play,
            board: Board::default(),
            pass_count: 0,
            player_count,
            status: GameStatus::ToPlay(to_play),
            scores: vec![0; player_count],
            racks,
        }
    }
    /// Gets the id of the current player.
    pub fn to_play(&self) -> PlayerId {
        self.to_play
    }
    /// Gets the id of the next player.
    pub fn next_player(&self) -> PlayerId {
        PlayerId((usize::from(self.to_play) + 1) % self.player_count)
    }
    /// Pops the most recent play from the history and undoes it.
    pub fn undo_play(&mut self) {
        todo!()
    }
    /// Gets an iterator over the player ids.
    pub fn player_ids(&self) -> impl Iterator<Item = PlayerId> {
        (0..self.player_count).map(PlayerId)
    }
    /// Borrows a player's rack by id.
    pub fn rack(&self, id: PlayerId) -> &Rack {
        &self.racks[usize::from(id)]
    }
    /// Gets a player's score by id.
    pub fn score(&self, id: PlayerId) -> usize {
        self.scores[usize::from(id)]
    }
    /// Borrows the current status of the game.
    pub fn status(&self) -> &GameStatus {
        &self.status
    }
    /// Checks whether the game is over.
    pub fn is_over(&self) -> bool {
        matches!(self.status(), GameStatus::Over(_, _))
    }
    /// Attempts to make a [`Play`].
    pub fn make_play(&mut self, play: Play) -> GameResult<()> {
        if self.is_over() {
            return Err(GameError::Over);
        }

        let id = usize::from(self.to_play());
        let rack = &mut self.racks[id];

        match &play {
            Play::Pass => self.pass_count += 1,
            Play::Redraw(tiles) => {
                // check number of tiles
                if !(1..=7).contains(&tiles.len()) {
                    return Err(GameError::RedrawCount);
                }

                // attempt to swap out tiles
                rack.exchange_tiles(tiles, &mut self.letter_bag)?;

                // not a pass so set pas count to zero
                self.pass_count = 0;
            }
            Play::Place(tile_positions) => {
                // check number of tiles
                if !(1..=7).contains(&tile_positions.len()) {
                    return Err(GameError::PlacementCount);
                }

                // check whether rack contains tiles
                if !rack.contains(tile_positions.iter().map(|(_, t)| *t)) {
                    return Err(GameError::NotInRack);
                }

                // attempt to make the placement
                let score = self.board.make_placement(tile_positions, self.word_tree)?;
                self.scores[id] += score;

                // remove letters from rack
                rack.remove(tile_positions.iter().map(|(_, t)| *t));

                // refill rack
                rack.refill(&mut self.letter_bag);

                // not a pass so set pas count to zero
                self.pass_count = 0;
            }
        };

        // update current player
        self.to_play = self.next_player();

        // Check whether any player has an empty rack
        let empty_rack = self
            .player_ids()
            .map(|id| self.rack(id))
            .any(|rack| rack.is_empty());

        // If there have been 6 rounds of passes in a row,
        // or a player has no letters on their rack,
        // then the game is over.
        let reason = if self.pass_count == 6 * self.player_count {
            Some(Reason::SixPasses)
        } else if empty_rack {
            Some(Reason::EmptyRack)
        } else {
            None
        };

        self.status = match reason {
            None => {
                // Game is ongoing
                GameStatus::ToPlay(self.to_play)
            }
            Some(reason) => {
                let mut rack_sum = 0;

                // Compute initial scores:
                // = (the running score of each player) - (the total of tiles on their rack)
                for id in self.player_ids() {
                    let id = usize::from(id);
                    let rack_total = self.racks[id]
                        .iter()
                        .map(|tile| tile.score())
                        .sum::<usize>();
                    let final_score = (self.scores[id] as i32 - rack_total as i32).max(0) as usize;

                    self.scores[id] = final_score as usize;

                    rack_sum += rack_total;
                }

                // Compute final scores:
                // the final score of any player with no remaining tiles is increased
                // by the sum of the remaining tiles
                for id in 0..self.player_count {
                    // if the player's rack was empty then this is true
                    if self.racks[id].is_empty() {
                        self.scores[id] += rack_sum;
                    }
                }

                let max_score = self.scores.iter().copied().max().unwrap_or(0);
                let winners = self
                    .player_ids()
                    .filter(|&id| self.scores[usize::from(id)] == max_score)
                    .collect();

                GameStatus::Over(winners, reason)
            }
        };

        Ok(())
    }
}
