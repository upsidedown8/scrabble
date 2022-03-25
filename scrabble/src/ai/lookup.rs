//! Precalculates the scores for vertical (perpendicular) words
//! with a single letter placed in each column.

use crate::{
    game::{board::CELLS, tile::Tile},
    util::{
        bitboard::BitBoard,
        fsm::{Fsm, StateId},
        grid::Grid,
        pos::{Col, Direction, Pos, Row},
        tile_counts::TileCounts,
    },
};
use std::collections::HashMap;

/// Vertical words can be considered seperately. Since placement
/// always occurs in a single row, only one tile can be placed in each
/// column. `Lookup` produces and stores a hashmap of legal tiles and their
/// corresponding scores for each position on the board.
#[derive(Debug)]
pub struct Lookup {
    above_or_below: BitBoard,
    lookup: Vec<HashMap<Tile, usize>>,
}
impl Lookup {
    /// Creates a lookup for the perpendicular direction (to the `grid`).
    /// `counts` are the frequencies of each tile on the rack.
    pub fn new<'a, F: Fsm<'a>>(fsm: &'a F, counts: &TileCounts, grid: &Grid) -> Self {
        LookupBuilder { fsm, counts, grid }.build()
    }
    /// Finds the score when a tile is placed on a square. If the word is invalid,
    /// returns `None`, if the word has only one letter returns `Some(0)`, otherwise
    /// returns the score of the word.
    pub fn score_tile(&self, pos: Pos, tile: Tile) -> Option<usize> {
        match self.above_or_below.is_set(pos) {
            true => self.lookup[usize::from(pos)].get(&tile).copied(),
            false => Some(0),
        }
    }
    /// Checks whether a postion has an existing tile above or below it.
    pub fn is_above_or_below(&self, pos: Pos) -> bool {
        self.above_or_below.is_set(pos)
    }
}

/// Struct used for recursively constructing a [`Lookup`].
struct LookupBuilder<'a, 'b, F> {
    fsm: &'a F,
    counts: &'b TileCounts,
    grid: &'b Grid,
}
impl<'a, 'b, F: Fsm<'a>> LookupBuilder<'a, 'b, F> {
    /// Constructs a `Lookup` from the builder.
    pub fn build(mut self) -> Lookup {
        let above_or_below = self.grid.occ().above_or_below();
        let mut lookup = (0..CELLS).map(|_| HashMap::default()).collect::<Vec<_>>();

        // Each column can be considered seperately. Considering
        // columns seperately also means that fewer fsm traversals
        // are required, as these words are vertical.
        for col in Col::iter() {
            let mut state = self.fsm.initial_state();
            let mut score = 0;

            // Go down the rows for the current column
            for pos in Row::iter().map(|row| Pos::from((row, col))) {
                match self.grid[pos] {
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
                        if above_or_below.is_set(pos) {
                            // try all tiles that could be placed here.
                            for (letter, next_state) in self.fsm.transitions(state) {
                                for tile in [Tile::Letter(letter), Tile::Blank(Some(letter))] {
                                    if self.counts.any(tile) {
                                        if let Some((tile, score)) =
                                            self.score(tile, score, pos, next_state)
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

        Lookup {
            above_or_below,
            lookup,
        }
    }

    /// Finds the scores for a vertical word from a position.
    fn score(
        &mut self,
        tile: Tile,
        score: usize,
        pos: Pos,
        state: StateId,
    ) -> Option<(Tile, usize)> {
        let (tile_m, multiplier) = pos.premium_multipliers();
        let mut score = score + tile_m * tile.score();
        let mut state = state;

        // keep following the word down the board until:
        //  - an empty square is encountered, or
        //  - the end of the board is encountered
        // skip `pos` as it was previously validated and scored.
        for pos in pos.project(Direction::South).skip(1) {
            match self.grid[pos] {
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
