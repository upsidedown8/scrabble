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
        pos::{Direction, Pos},
        tile_counts::TileCounts,
    },
};

/// Gets a list containing all moves for the board position.
pub fn gen<'a>(board: &Board, rack: &Rack, fsm: &'a impl Fsm<'a>) -> Vec<ScoredPlay> {
    let mut plays = vec![];

    gen_horizontal(&mut plays, board, rack, fsm);
    // gen_vertical(&mut plays, board, rack, fsm);

    plays
}

/// Adds all horizontal moves in a position to the list of plays.
fn gen_horizontal<'a>(
    plays: &mut Vec<ScoredPlay>,
    board: &Board,
    rack: &Rack,
    fsm: &'a impl Fsm<'a>,
) {
    let get_cell = |pos: Pos| board.get(pos);
    let map_pos = |pos: Pos| pos;
    let &occ_h = board.occ_h();

    MoveGen::new(rack, occ_h, fsm, get_cell, map_pos).gen(plays);
}

/// Adds all vertical moves in a position to the list of plays.
fn gen_vertical<'a>(
    plays: &mut Vec<ScoredPlay>,
    board: &Board,
    rack: &Rack,
    fsm: &'a impl Fsm<'a>,
) {
    let get_cell = |pos: Pos| board.get(pos.anti_clockwise90());
    let map_pos = |pos: Pos| pos.anti_clockwise90();
    let &occ_v = board.occ_v();

    MoveGen::new(rack, occ_v, fsm, get_cell, map_pos).gen(plays);
}

/// Wrapper type for a generated `Play` and its score.
#[derive(Debug)]
pub struct ScoredPlay(pub Play, pub usize);
impl ScoredPlay {
    /// Creates a new scored play from tile positions and its score.
    pub fn new<MapPos>(tiles: &[(Pos, Tile)], score: usize, map_pos: MapPos) -> Self
    where
        MapPos: Fn(Pos) -> Pos,
    {
        let play = Play::Place(
            tiles
                .iter()
                .map(|&(pos, tile)| (map_pos(pos), tile))
                .collect(),
        );

        ScoredPlay(play, score)
    }
}

/// A struct that stores recursive state so that it can be
/// more easily passed to other methods.
struct WordState {
    state: StateId,
    score: usize,
    multiplier: usize,
    connected: bool,
}

/// Generates either horizontal or vertical moves.
/// *   `GetCell` gets the actual optional tile on the board from the rotated position
/// *   `MapPos` performs a counter rotation if neccesary.
#[derive(Debug)]
struct MoveGen<'a, F, GetCell, MapPos> {
    fsm: &'a F,
    get_cell: GetCell,
    map_pos: MapPos,
    lookup: Lookup,

    occ: BitBoard,
    above_or_below: BitBoard,
    illegal_ends: BitBoard,

    stack: Vec<(Pos, Tile)>,
    counts: TileCounts,
}

impl<'a, F, GetCell, MapPos> MoveGen<'a, F, GetCell, MapPos>
where
    F: Fsm<'a>,
    GetCell: Fn(Pos) -> Option<Tile> + Copy,
    MapPos: Fn(Pos) -> Pos + Copy,
{
    /// Creates a new [`MoveGen`].
    pub fn new(rack: &Rack, occ: BitBoard, fsm: &'a F, get_cell: GetCell, map_pos: MapPos) -> Self {
        Self {
            fsm,
            get_cell,
            map_pos,
            lookup: Lookup::new(fsm, get_cell, rack.tile_counts(), occ),

            occ,
            above_or_below: occ.above_or_below(),
            illegal_ends: occ.west(),

            stack: vec![],
            counts: *rack.tile_counts(),
        }
    }

    /// Adds all moves for a position to the list.
    pub fn gen(&mut self, plays: &mut Vec<ScoredPlay>) {
        for start in util::possible_starts_h(self.occ, self.counts.len()) {
            self.gen_recursive(
                plays,
                Some(start),
                WordState {
                    state: self.fsm.initial_state(),
                    score: 0,
                    multiplier: 1,
                    connected: false,
                },
            );
        }
    }

    /// Recursively traverses possible moves and adds them to the list.
    fn gen_recursive(&mut self, plays: &mut Vec<ScoredPlay>, pos: Option<Pos>, ws: WordState) {
        self.check_position(plays, &ws);

        if let Some(pos) = pos {
            let next_pos = pos.dir(Direction::East);

            match (self.get_cell)(pos) {
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
        plays: &mut Vec<ScoredPlay>,
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
        plays: &mut Vec<ScoredPlay>,
        ws: WordState,
        next_pos: Option<Pos>,
        pos: Pos,
    ) {
        // try each transition from this state.
        for (letter, next_state) in self.fsm.transitions(ws.state) {
            for tile in [Tile::Letter(letter), Tile::Blank(Some(letter))] {
                if let Some(perpendicular_score) = self.lookup.score_tile(pos, tile) {
                    if self.counts.any(tile) {
                        self.counts.remove_one(letter);
                        self.stack.push((pos, tile));

                        let (tile_m, word_m) = pos.premium_multipliers();

                        self.gen_recursive(
                            plays,
                            next_pos,
                            WordState {
                                state: next_state,
                                score: ws.score + tile_m * tile.score() + perpendicular_score,
                                multiplier: ws.multiplier * word_m,
                                connected: ws.connected || self.lookup.is_above_or_below(pos),
                            },
                        );

                        self.stack.pop();
                        self.counts.insert_one(letter);
                    }
                }
            }
        }
    }

    /// Checks whether a point in the recursive stack is a valid move,
    /// and if so adds it to the list.
    fn check_position(&self, plays: &mut Vec<ScoredPlay>, ws: &WordState) {
        if ws.connected && self.fsm.is_terminal(ws.state) {
            // check that the final stack item does not have
            // a disallowed end position & that stack is not empty.
            if let Some(&(pos, _)) = self.stack.last() {
                // prevents illegal plays if the end of a word is adjacent to a tile.
                if !self.illegal_ends.is_set(pos) {
                    // prevents doubled up moves from horizontal and vertical generation.
                    // if there is only one tile, it must not be adjacent.
                    if self.stack.len() > 1 || !self.above_or_below.is_set(pos) {
                        let all_tiles_bonus = match self.stack.len() {
                            7 => 50,
                            _ => 0,
                        };

                        plays.push(ScoredPlay::new(
                            &self.stack,
                            ws.score * ws.multiplier + all_tiles_bonus,
                            self.map_pos,
                        ));
                    }
                }
            }
        }
    }
}
