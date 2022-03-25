//! Move generator implementation. Use the top level function [`gen`]
//! to get a list of scored plays.

use crate::{
    ai::lookup::Lookup,
    game::{
        board::Board,
        play::Play,
        rack::Rack,
        tile::{Letter, Tile},
    },
    util::{
        self,
        bitboard::BitBoard,
        fsm::{Fsm, StateId},
        grid::Grid,
        pos::{Direction, Pos},
        tile_counts::TileCounts,
    },
};

/// Adds all moves for the board position to `plays`. (Clears `plays` first).
pub fn gen<'a>(board: &Board, rack: &Rack, fsm: &'a impl Fsm<'a>, plays: &mut Vec<GeneratedPlay>) {
    plays.clear();
    MoveGen::new(rack, board.grid_v(), fsm).gen(plays);
    MoveGen::new(rack, board.grid_h(), fsm).gen(plays);
}

/// Stores a generated play and details that can be used to
/// score the play.
#[derive(Debug)]
pub struct GeneratedPlay {
    /// The play (tile positions).
    pub tile_positions: Vec<(Pos, Tile)>,
    /// The total score of the play.
    pub score: usize,
    /// The number of perpendicular words formed.
    pub cross_count: usize,
    /// The length of the primary word.
    pub len: usize,
}
impl From<GeneratedPlay> for Play {
    fn from(gen_play: GeneratedPlay) -> Self {
        Play::Place(gen_play.tile_positions)
    }
}
impl GeneratedPlay {
    /// Converts to a [`Play`].
    pub fn play(self) -> Play {
        Play::from(self)
    }
}

/// A struct that stores recursive state so that it can be
/// more easily passed to other methods.
struct WordState {
    state: StateId,
    score: usize,
    cross_count: usize,
    len: usize,
    multiplier: usize,
    connected: bool,
}

/// Generates moves for a position.
#[derive(Debug)]
struct MoveGen<'a, 'b, F> {
    fsm: &'a F,
    grid: &'b Grid,
    lookup: Lookup,

    occ: BitBoard,
    illegal_ends: BitBoard,

    stack: Vec<(Pos, Tile)>,
    counts: TileCounts,
}

impl<'a, 'b, F> MoveGen<'a, 'b, F>
where
    F: Fsm<'a>,
{
    /// Creates a new [`MoveGen`].
    pub fn new(rack: &Rack, grid: &'b Grid, fsm: &'a F) -> Self {
        let &occ = grid.occ();
        let &counts = rack.tile_counts();
        let lookup = Lookup::new(fsm, &counts, grid);

        Self {
            fsm,
            grid,
            lookup,

            occ,
            illegal_ends: occ.west(),

            stack: vec![],
            counts,
        }
    }
    /// Adds all moves for a position to the list.
    pub fn gen(mut self, plays: &mut Vec<GeneratedPlay>) {
        for start in util::possible_starts_h(self.occ, self.counts.len()) {
            self.gen_recursive(
                plays,
                Some(start),
                WordState {
                    state: self.fsm.initial_state(),
                    score: 0,
                    cross_count: 0,
                    len: 0,
                    multiplier: 1,
                    connected: false,
                },
            );
        }
    }
    /// Recursively traverses possible moves and adds them to the list.
    fn gen_recursive(&mut self, plays: &mut Vec<GeneratedPlay>, pos: Option<Pos>, ws: WordState) {
        self.check_position(plays, &ws);

        if let Some(pos) = pos {
            let next_pos = pos.dir(Direction::East);

            match self.grid[pos] {
                // Already a tile: see whether traversal can continue (no branching).
                // Add the tile score to the total score.
                Some(tile @ Tile::Letter(letter) | tile @ Tile::Blank(Some(letter))) => {
                    self.occupied_square(plays, ws, next_pos, tile, letter)
                }
                // Empty square at current position, so premium positions apply. Try all
                // possible tiles, with reference to `lookup` (to ensure that any vertical
                // words are also valid).
                _ => self.empty_square(plays, ws, next_pos, pos),
            }
        }
    }
    /// Handles the case in `gen_recursive` where the board square is
    /// already occupied.
    fn occupied_square(
        &mut self,
        plays: &mut Vec<GeneratedPlay>,
        ws: WordState,
        next_pos: Option<Pos>,
        tile: Tile,
        letter: Letter,
    ) {
        if let Some(next_state) = self.fsm.traverse_from(ws.state, letter) {
            self.gen_recursive(
                plays,
                next_pos,
                WordState {
                    state: next_state,
                    score: ws.score + tile.score(),
                    len: ws.len + 1,
                    cross_count: ws.cross_count,
                    multiplier: ws.multiplier,
                    connected: true,
                },
            );
        }
    }
    /// Handles the case in `gen_recursive` where the board square is
    /// not yet occupied.
    fn empty_square(
        &mut self,
        plays: &mut Vec<GeneratedPlay>,
        ws: WordState,
        next_pos: Option<Pos>,
        pos: Pos,
    ) {
        // try each transition from this state.
        for (letter, next_state) in self.fsm.transitions(ws.state) {
            for tile in [Tile::Letter(letter), Tile::Blank(Some(letter))] {
                if let Some(perpendicular_score) = self.lookup.score_tile(pos, tile) {
                    if self.counts.any(tile) {
                        self.counts.remove_one(tile);
                        self.stack.push((pos, tile));

                        let (tile_m, word_m) = pos.premium_multipliers();

                        self.gen_recursive(
                            plays,
                            next_pos,
                            WordState {
                                state: next_state,
                                score: ws.score + tile_m * tile.score() + perpendicular_score,
                                // increment the cross count if a perpendicular
                                // word with a non zero score is placed.
                                cross_count: ws.cross_count
                                    + if perpendicular_score > 0 { 1 } else { 0 },
                                len: ws.len + 1,
                                multiplier: ws.multiplier * word_m,
                                connected: ws.connected || self.lookup.is_above_or_below(pos),
                            },
                        );

                        self.stack.pop();
                        self.counts.insert_one(tile);
                    }
                }
            }
        }
    }
    /// Checks whether a point in the recursive stack is a valid move,
    /// and if so adds it to the list.
    fn check_position(&self, plays: &mut Vec<GeneratedPlay>, ws: &WordState) {
        // check that the word is connected and is valid.
        if ws.connected && self.fsm.is_terminal(ws.state) {
            // check that the final stack item does not have
            // a disallowed end position & that stack is not empty.
            if let Some(&(pos, _)) = self.stack.last() {
                // prevents illegal plays if the end of a word is adjacent to a tile.
                if !self.illegal_ends.is_set(pos) {
                    // prevents doubled up moves from horizontal and vertical generation.
                    // if there is only one tile, it must not be adjacent.
                    if self.stack.len() > 1 || !self.lookup.is_above_or_below(pos) {
                        self.add_play(plays, ws)
                    }
                }
            }
        }
    }
    /// Adds a play to the list.
    fn add_play(&self, plays: &mut Vec<GeneratedPlay>, ws: &WordState) {
        let all_tiles_bonus = match self.stack.len() {
            7 => 50,
            _ => 0,
        };

        plays.push(GeneratedPlay {
            tile_positions: self
                .stack
                .iter()
                // maps the position back to the horizontal coordinate.
                .map(|&(pos, tile)| (self.grid.map_pos(pos), tile))
                .collect(),
            score: ws.score * ws.multiplier + all_tiles_bonus,
            cross_count: 0,
            len: ws.len,
        });
    }
}
