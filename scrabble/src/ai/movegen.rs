//! Move generator implementation.

use crate::{
    game::{board::Board, play::Play, rack::Rack, tile::Tile},
    util::{
        bitboard::BitBoard,
        fsm::{Fsm, StateId},
        pos::{Col, Direction, Pos, Row},
        tile_counts::TileCounts,
    },
};
use std::{collections::HashMap, iter::successors};

/// An iterator over the moves for a board position.
pub struct Moves<'a, F> {
    fsm: &'a F,
    board: &'a Board,
    rack: &'a Rack,
}

impl<'a, F: Fsm<'a>> Moves<'a, F> {
    /// Creates a new iterator over the horizontal moves on the board.
    pub fn new(fsm: &'a F, board: &'a Board, rack: &'a Rack) -> Self {
        Self { fsm, board, rack }
    }

    /// Returns the list of horizontal moves for a position.
    pub fn moves(&self) -> Vec<(Play, usize)> {
        let &occ_h = self.board.occ_h();
        let above_or_below = occ_h.above_or_below();
        let illegal_ends = occ_h.west();

        let starts = Self::all_starts_h(*self.board.occ_h(), self.rack.len());
        let lookup = self.lookup_v();
        let mut plays = vec![];
        let mut stack = vec![];
        let mut counts = *self.rack.tile_counts();

        for start in starts {
            self.add_moves(
                &mut plays,
                &lookup,
                &mut counts,
                &mut stack,
                &above_or_below,
                &illegal_ends,
                Some(start),
                self.fsm.initial_state(),
                0,
                1,
                false,
            );
        }

        plays
    }
    /// Recursively traverses possible moves and adds them to the list.
    #[allow(clippy::too_many_arguments)]
    fn add_moves(
        &self,
        plays: &mut Vec<(Play, usize)>,
        lookup: &[HashMap<Tile, usize>],
        counts: &mut TileCounts,
        stack: &mut Vec<(Pos, Tile)>,
        above_or_below: &BitBoard,
        illegal_ends: &BitBoard,
        pos: Option<Pos>,
        state: StateId,
        score: usize,
        multiplier: usize,
        connected: bool,
    ) {
        if connected && self.fsm.is_terminal(state) {
            // check that the final stack item does not have
            // a disallowed end position & that stack is not empty.
            if let Some(&(pos, _)) = stack.last() {
                if !illegal_ends.is_bit_set(pos) {
                    plays.push((Play::Place(stack.clone()), score * multiplier));
                }
            }
        }

        if let Some(pos) = pos {
            let next_pos = pos.dir(Direction::East);

            match self.board.at(pos) {
                // Already a tile: see whether traversal can continue (no branching).
                // Add the tile score to the total score.
                Some(tile @ Tile::Letter(letter) | tile @ Tile::Blank(Some(letter))) => {
                    let score = score + tile.score();

                    if let Some(next_state) = self.fsm.traverse_from(state, letter) {
                        self.add_moves(
                            plays,
                            lookup,
                            counts,
                            stack,
                            above_or_below,
                            illegal_ends,
                            next_pos,
                            next_state,
                            score,
                            multiplier,
                            true,
                        );
                    }
                }
                // Empty square at current position, so premium positions apply. Try all
                // possible tiles, with reference to `lookup` (to ensure that any vertical
                // words are also valid).
                _ => {
                    let (tile_m, word_m) = pos.premium_multipliers();
                    let has_adjacant = above_or_below.is_bit_set(pos);

                    // try each transition from this state.
                    for (letter, next_state) in self.fsm.transitions(state) {
                        for tile in [Tile::Letter(letter), Tile::Blank(Some(letter))] {
                            let extra_score = match has_adjacant {
                                false => 0,
                                true => match lookup[usize::from(pos)].get(&tile) {
                                    Some(&score) => score,
                                    // not a valid tile, continue to next loop.
                                    _ => continue,
                                },
                            };

                            if counts.any(tile) {
                                counts.remove_one(letter);
                                stack.push((pos, tile));

                                self.add_moves(
                                    plays,
                                    lookup,
                                    counts,
                                    stack,
                                    above_or_below,
                                    illegal_ends,
                                    next_pos,
                                    next_state,
                                    extra_score + score + tile_m * tile.score(),
                                    multiplier * word_m,
                                    connected || has_adjacant,
                                );

                                stack.pop();
                                counts.insert_one(letter);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Gets a bitboard containing the set of squares on which a
    /// horizontal word could start.
    fn all_starts_h(occ_h: BitBoard, rack_len: usize) -> BitBoard {
        // find all word stems. that is: all tiles shifted up,down and left,
        // excluding existing tiles.
        let mut stems = (occ_h.north() | occ_h.south() | occ_h.west()) & !occ_h;
        // shift and add `stems` to the left (rack_len - 1) times, as one shift
        // was already performed above.
        for _ in 0..rack_len - 1 {
            stems |= stems.west();
        }

        // find the starts of all existing words
        let starts = occ_h.word_starts_h();

        // final set of starts is the `stems` plus the `starts`, without any
        // current tiles that do not start words, minus the rightmost column
        // (as no word can start there).
        ((stems & !occ_h) | starts) & !occ_h.east() & !BitBoard::rightmost_col()
    }

    /// Vertical words can be considered seperately. Since placement
    /// always occurs in a single row, only 1 tile can be placed in each
    /// column. This function produces a hashmap of legal tiles and their
    /// corresponding scores for each position on the board.
    fn lookup_v(&self) -> Vec<HashMap<Tile, usize>> {
        let counts = self.rack.tile_counts();
        let mut lookup: Vec<_> = (0..225).map(|_| HashMap::new()).collect();
        let above_or_below = self.board.occ_h().above_or_below();

        // Each column can be considered seperately. Considering
        // columns seperately also means that fewer fsm traversals
        // are required, as these words are vertical.
        for col in Col::iter() {
            let mut state = self.fsm.initial_state();
            let mut score = 0;

            // Go down the rows for the current column
            for pos in Row::iter().map(|row| Pos::from((row, col))) {
                match self.board.at(pos) {
                    // already a tile at this position, update score and state.
                    Some(tile @ Tile::Letter(letter) | tile @ Tile::Blank(Some(letter))) => {
                        // since the tile has already been placed, the path should be in the fsm.
                        state = self.fsm.traverse_from(state, letter).expect("a valid word");
                        // add the tile score but do not apply any premiums.
                        score += tile.score();
                    }
                    _ => {
                        // if the position is not directly above or below
                        // an existing square then words placed there ignore
                        // the map for that square.
                        if above_or_below.is_bit_set(pos) {
                            // try all tiles that could be placed here.
                            for (letter, next_state) in self.fsm.transitions(state) {
                                for tile in [Tile::Letter(letter), Tile::Blank(Some(letter))] {
                                    if counts.any(tile) {
                                        if let Some((tile, score)) =
                                            self.score_v(tile, score, pos, next_state)
                                        {
                                            lookup[usize::from(pos)].insert(tile, score);
                                        }
                                    }
                                }
                            }
                        }

                        // reset the state and values as there is a break in the column.
                        state = self.fsm.initial_state();
                        score = 0;
                    }
                }
            }
        }

        lookup
    }
    /// Finds the scores for a vertical word from a position.
    fn score_v(&self, tile: Tile, score: usize, pos: Pos, state: StateId) -> Option<(Tile, usize)> {
        let (tile_m, multiplier) = pos.premium_multipliers();
        let mut score = score + tile_m * tile.score();
        let mut state = state;

        // keep following the word down the board until:
        //  - an empty square is encountered, or
        //  - the end of the board is encountered
        // skip `pos` as it was previously validated and scored.
        for pos in successors(pos.dir(Direction::South), |p| p.dir(Direction::South)) {
            match self.board.at(pos) {
                Some(tile @ Tile::Letter(letter) | tile @ Tile::Blank(Some(letter))) => {
                    // these tiles are already placed so premium does not apply
                    score += tile.score();
                    state = self.fsm.traverse_from(state, letter)?;
                }
                _ => break,
            }
        }

        // Only a valid word if the final state is terminal.
        match self.fsm.is_terminal(state) {
            true => Some((tile, score * multiplier)),
            false => None,
        }
    }
}
