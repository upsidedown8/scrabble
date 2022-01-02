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
use rand::Rng;
use std::collections::HashMap;

pub mod bitboard;
pub mod board;
pub mod error;
pub mod letter_bag;
pub mod play;
pub mod pos;
pub mod rack;
pub mod tile;
pub mod word_tree;

pub enum GameStatus {
    Winner(Player, usize),
    ToPlay(Player),
}

/// Used to identify a player.
#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerId(usize);

/// Models the game state for each player. Namely their Rack
/// and their score.
#[derive(Debug)]
pub struct Player {
    rack: Rack,
    score: usize,
}
impl Player {
    /// Creates a new `Player`, using the `letter_bag` to fill their rack.
    pub fn new(letter_bag: &mut LetterBag) -> Self {
        Self {
            rack: Rack::new(letter_bag),
            score: 0,
        }
    }
    /// Borrows the player's rack mutably
    pub fn rack_mut(&mut self) -> &mut Rack {
        &mut self.rack
    }
    /// Gets the player's score.
    pub fn score(&self) -> usize {
        self.score
    }
    /// Adds a quantity to the player's score.
    pub fn add_score(&mut self, score: usize) {
        self.score += score;
    }
    /// Subtracts a quantity from the player's score.
    pub fn sub_score(&mut self, score: usize) {
        self.score -= score;
    }
}

/// Top level struct allowing for management of the entire
/// game. Manages players, all state, and determines when the
/// game is over, calculating scores and determining the winner.
pub struct Game<'a> {
    word_tree: &'a WordTree,
    board: Board,
    letter_bag: LetterBag,
    to_play: PlayerId,
    players: HashMap<PlayerId, Player>,
    history: Vec<(PlayerId, Play)>,
    pass_count: usize,
    player_count: usize,
}

impl<'a> Game<'a> {
    /// Constructs a new [`Game`] from a borrowed `word_tree` and the number
    /// of players.
    pub fn new(word_tree: &'a WordTree, player_count: usize) -> Self {
        let mut letter_bag = LetterBag::default();

        let to_play = PlayerId(rand::thread_rng().gen_range(0..player_count));
        let players = (0..player_count)
            .map(|id| (PlayerId(id), Player::new(&mut letter_bag)))
            .collect::<HashMap<_, _>>();

        Self {
            word_tree,
            letter_bag,
            to_play,
            players,
            board: Board::default(),
            history: vec![],
            pass_count: 0,
            player_count,
        }
    }
    /// Attempts to make a [`Play`].
    pub fn make_play(&mut self, play: Play) -> GameResult<()> {
        match play {
            Play::Pass => self.pass_count += 1,
            Play::Redraw(tiles) => {
                if !(1..=7).contains(&tiles.len()) {
                    return Err(GameError::RedrawCount);
                }
            }
            Play::Place(tile_positions) => {
                let score = self.board.make_placement(tile_positions, self.word_tree)?;

                if let Some(player) = self.players.get_mut(&self.to_play) {
                    player.add_score(score);
                }

                println!("{}", self.board);
            }
        };

        self.to_play = self.next_player();

        todo!()
    }
    /// Gets the id of the current player.
    pub fn to_play(&self) -> PlayerId {
        self.to_play
    }
    /// Gets the id of the next player.
    pub fn next_player(&self) -> PlayerId {
        PlayerId((self.to_play.0 + 1) % self.player_count)
    }
    /// Pops the most recent play from the history and undoes it.
    pub fn undo_play(&mut self) {
        todo!()
    }
    /// Gets the current status of the game.
    pub fn status(&self) -> GameStatus {
        todo!()
    }
}
