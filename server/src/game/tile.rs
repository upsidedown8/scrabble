use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Letter(usize);

impl Letter {
    pub fn new(ch: char) -> Option<Self> {
        match ch {
            'a'..='z' => Some(Letter(ch as usize - 97)),
            'A'..='Z' => Some(Letter(ch as usize - 65)),
            _ => None,
        }
    }
}
impl From<usize> for Letter {
    fn from(v: usize) -> Self {
        Self(v % 26)
    }
}
impl From<Letter> for usize {
    fn from(letter: Letter) -> Self {
        letter.0
    }
}
impl Display for Letter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.0 + 65) as u8 as char)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tile {
    Letter(Letter),
    Blank(Option<Letter>),
}

impl From<Option<Letter>> for Tile {
    fn from(op: Option<Letter>) -> Self {
        Self::Blank(op)
    }
}
impl From<Letter> for Tile {
    fn from(letter: Letter) -> Self {
        Self::Letter(letter)
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Letter(l) => write!(f, " {} ", l),
            Tile::Blank(Some(l)) => write!(f, "({})", l),
            Tile::Blank(_) => write!(f, "( )"),
        }
    }
}
