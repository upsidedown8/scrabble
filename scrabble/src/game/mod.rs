//! Module for handling abstract game representation and player
//! interaction (uncoupled from DB, UI and API).
//!
//! Game logic is shared between the client and server, so that
//! API calls can be minimised, (only for actually making moves)
//! by performing the majority of validation on the client side.
//! The types exposed in this module are also useful for modelling
//! state for the UI.

use crate::{
    error::{GameError, GameResult},
    game::{board::Board, letter_bag::LetterBag, play::Play, rack::Rack, tile::Tile},
    util::{fsm::Fsm, pos::Pos},
};
use serde::{Deserialize, Serialize};

pub mod board;
pub mod letter_bag;
pub mod play;
pub mod rack;
pub mod tile;

/// Top level struct allowing for management of the entire
/// game. Manages players, all state, and determines when the
/// game is over, calculating scores and determining the winner.
#[derive(Debug)]
pub struct Game {
    board: Board,
    letter_bag: LetterBag,
    players: Vec<Player>,
    to_play: PlayerNum,
    status: GameStatus,
}

/// Models a scrabble player.
#[derive(Debug)]
pub struct Player {
    rack: Rack,
    score: usize,
    pass_count: usize,
}
impl Player {
    /// Gets the player's rack.
    pub fn rack(&self) -> &Rack {
        &self.rack
    }
    /// Gets the player's score.
    pub fn score(&self) -> usize {
        self.score
    }
}

/// The current state of the game.
#[derive(Clone, Debug)]
pub enum GameStatus {
    /// One or more players have won
    Over(GameOver),
    /// The game is ongoing.
    ToPlay(PlayerNum),
}
impl GameStatus {
    /// Checks whether the game is over.
    pub fn is_over(&self) -> bool {
        matches!(self, GameStatus::Over(_))
    }
}

/// Stores the final scores and the outcome of the game.
#[derive(Clone, Debug)]
pub struct GameOver {
    max_score: usize,
    scores: Vec<usize>,
    reason: GameOverReason,
}
impl GameOver {
    /// Computes the final scores from the game state.
    pub fn new(reason: GameOverReason, players: &[Player], last_player: PlayerNum) -> Self {
        let mut scores = vec![0; players.len()];
        let mut overall_rack_sum = 0;

        // First calculate the initial scores for all players, as
        //     (current running total) - (sum of tiles on rack)
        for (idx, player) in players.iter().enumerate() {
            let rack_sum = player.rack.tile_sum();
            scores[idx] = player.score.saturating_sub(rack_sum);
            overall_rack_sum += rack_sum;
        }

        // Then calculate the final score for the player that ended the game,
        // by adding `overall_rack_total` to their score.
        scores[usize::from(last_player)] += overall_rack_sum;

        let &max_score = scores.iter().max().unwrap_or(&0);

        Self {
            max_score,
            scores,
            reason,
        }
    }
    /// Gets the score for a particular player.
    pub fn score(&self, player_num: PlayerNum) -> usize {
        self.scores[usize::from(player_num)]
    }
    /// Gets the maximum score achieved.
    pub fn max_score(&self) -> usize {
        self.max_score
    }
    /// Gets the reason that the game ended.
    pub fn reason(&self) -> GameOverReason {
        self.reason
    }
    /// Gets an iterator over the winning players.
    pub fn winners(&self) -> impl Iterator<Item = (PlayerNum, usize)> + '_ {
        self.final_scores()
            .filter(|&(_, score)| score == self.max_score)
    }
    /// Gets an iterator over the losing players.
    pub fn losers(&self) -> impl Iterator<Item = (PlayerNum, usize)> + '_ {
        self.final_scores()
            .filter(|&(_, score)| score < self.max_score)
    }
    /// Gets an iterator over (player number, score) tuples.
    pub fn final_scores(&self) -> impl Iterator<Item = (PlayerNum, usize)> + '_ {
        PlayerNum::iter(self.scores.len()).zip(self.scores.iter().copied())
    }
}

/// The reason that the game has ended.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GameOverReason {
    /// A player has emptied their rack with no letters remaining in the bag.
    EmptyRack,
    /// A player has passed their turn twice in a row.
    TwoPasses,
}

/// Used to identify players within a [`Game`]. Since
/// the implementation is decoupled from any actual data,
/// the server/client has to handle how assigned [`PlayerNum`]
/// values relate to the actual players.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PlayerNum(usize);
impl PlayerNum {
    /// The first player.
    pub fn first() -> Self {
        Self(0)
    }
    /// Gets the next player, wrapping around at the end.
    pub fn next(self, player_count: usize) -> Self {
        let PlayerNum(val) = self;

        Self((val + 1) % player_count)
    }
    /// Gets an iterator over [`PlayerNum`]s.
    pub fn iter(player_count: usize) -> impl Iterator<Item = Self> {
        (0..player_count).map(Self)
    }
}
impl From<PlayerNum> for usize {
    fn from(PlayerNum(num): PlayerNum) -> Self {
        num
    }
}

impl Game {
    /// Constructs a new [`Game`] from the number of players.
    pub fn new(player_count: usize) -> Self {
        let mut letter_bag = LetterBag::default();

        let players = (0..player_count)
            .map(|_| Player {
                rack: Rack::new(&mut letter_bag),
                score: 0,
                pass_count: 0,
            })
            .collect();

        Self {
            letter_bag,
            to_play: PlayerNum::first(),
            board: Board::default(),
            status: GameStatus::ToPlay(PlayerNum::first()),
            players,
        }
    }
    /// Gets the next player number.
    pub fn to_play(&self) -> Option<PlayerNum> {
        match self.status() {
            GameStatus::Over(_) => None,
            GameStatus::ToPlay(to_play) => Some(*to_play),
        }
    }
    /// Gets the number of tiles left in the letter bag.
    pub fn letter_bag_len(&self) -> usize {
        self.letter_bag.len()
    }
    /// Gets the game state for a player.
    pub fn player(&self, player_num: PlayerNum) -> &Player {
        &self.players[usize::from(player_num)]
    }
    /// The current status of the game.
    pub fn status(&self) -> &GameStatus {
        &self.status
    }
    /// Borrows the board.
    pub fn board(&self) -> &Board {
        &self.board
    }
    /// Gets the number of players.
    pub fn player_count(&self) -> usize {
        self.players.len()
    }
    /// Gets an iterator over player numbers for the game.
    pub fn player_nums(&self) -> impl Iterator<Item = PlayerNum> {
        PlayerNum::iter(self.player_count())
    }

    /// Attempts to make a [`Play`].
    pub fn make_play<'a, F: Fsm<'a>>(&mut self, play: &Play, fsm: &F) -> GameResult<()> {
        // Return early if the game is over.
        if self.status().is_over() {
            return Err(GameError::Over);
        }

        // make the play.
        match play {
            Play::Pass => self.pass(),
            Play::Redraw(tiles) => self.redraw(tiles)?,
            Play::Place(tile_positions) => self.place(fsm, tile_positions)?,
        }

        // update current player & status
        let previous = self.to_play;
        self.to_play = self.to_play.next(self.player_count());
        self.status = self.next_status(previous);

        Ok(())
    }

    /// Makes a [`Play::Redraw`] play.
    fn redraw(&mut self, tiles: &[Tile]) -> GameResult<()> {
        let player = &mut self.players[usize::from(self.to_play)];

        // attempt to swap out tiles
        player.rack.exchange_tiles(tiles, &mut self.letter_bag)?;
        player.pass_count = 0;

        Ok(())
    }
    /// Makes a [`Play::Pass`] play.
    fn pass(&mut self) {
        let player = &mut self.players[usize::from(self.to_play)];

        // The player has passed, so update their count.
        player.pass_count += 1;
    }
    /// Makes a [`Play::Place`] play.
    fn place<'a, F: Fsm<'a>>(&mut self, fsm: &F, tile_positions: &[(Pos, Tile)]) -> GameResult<()> {
        let player = &mut self.players[usize::from(self.to_play)];

        // check that the player has enough tiles.
        if !player.rack.contains(tile_positions.iter().map(|&(_, t)| t)) {
            return Err(GameError::NotInRack);
        }

        // attempt to make the placement
        let score = self.board.make_placement(tile_positions, fsm)?;

        // update player data
        player.pass_count = 0;
        player.score += score;
        player.rack.remove(tile_positions.iter().map(|(_, t)| *t));
        player.rack.refill(&mut self.letter_bag);

        Ok(())
    }

    /// Determines the next game status.
    fn next_status(&self, previous: PlayerNum) -> GameStatus {
        let previous_player = &self.players[usize::from(previous)];

        if previous_player.pass_count >= 2 {
            // The game ends if the most recent player has passed twice
            // in a row.
            let game_over = GameOver::new(GameOverReason::TwoPasses, &self.players, previous);
            GameStatus::Over(game_over)
        } else if previous_player.rack.is_empty() {
            // The game ends if the most recent player has emptied their rack.
            let game_over = GameOver::new(GameOverReason::EmptyRack, &self.players, previous);
            GameStatus::Over(game_over)
        } else {
            // Otherwise the game is ongoing.
            GameStatus::ToPlay(self.to_play)
        }
    }
}
