//! Module representing a [`Play`] (move) made by a player.

use std::{collections::HashSet, iter};

use super::{
    bitboard::BitBoard,
    pos::{Direction, Pos},
    tile::{Letter, Tile},
};

pub struct Word {
    letters: Vec<Letter>,
    dir: Direction,
    start: Pos,
}

impl Word {
    pub fn new<T>(word: &str, dir: Direction, start: T) -> Option<Self>
    where
        T: Into<Pos>,
    {
        let start: Pos = start.into();
        let letters = word.chars().filter_map(Letter::new).collect();

        if start.offset(dir, word.chars().count()).is_some() {
            Some(Self {
                letters,
                dir,
                start,
            })
        } else {
            None
        }
    }
    pub fn tiles(&self) -> impl Iterator<Item = (Pos, Tile)> + '_ {
        iter::successors(Some(self.start), |p| p.offset(self.dir, 1))
            .zip(self.letters.iter().map(|l| Tile::Letter(*l)))
    }
}

pub struct Play {
    tiles: HashSet<(Pos, Tile)>,
    occupancy: BitBoard,
}

impl Play {
    pub fn from_words<I>(words: I) -> Self
    where
        I: Iterator<Item = Word>,
    {
        // collect all tiles from words
        let mut tiles = vec![];
        for w in words {
            for t in w.tiles() {
                tiles.push(t);
            }
        }

        Self::from_tiles(tiles.into_iter())
    }
    pub fn from_tiles<I>(tiles: I) -> Self
    where
        I: Iterator<Item = (Pos, Tile)>,
    {
        let tiles: HashSet<_> = tiles.collect();
        let occupancy = tiles.iter().fold(BitBoard::default(), |mut bb, (pos, _)| {
            bb.set_bit(*pos);
            bb
        });

        Self { tiles, occupancy }
    }
    pub fn tiles(&self) -> impl Iterator<Item = &(Pos, Tile)> + '_ {
        self.tiles.iter()
    }
    pub fn occupancy(&self) -> BitBoard {
        self.occupancy
    }
}
