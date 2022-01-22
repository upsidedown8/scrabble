use common::game::{
    play::{Play, Word},
    pos::{Col, Direction, Row},
    word_tree::WordTree,
    Game,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut word_tree = WordTree::default();
    let file = File::open("../server/data/words.txt").unwrap();
    let rdr = BufReader::new(file);

    for line in rdr.lines().flatten() {
        word_tree.insert(line.trim());
    }

    let mut game = Game::new(&word_tree, 2);

    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    while !game.is_over() {
        println!("{}", game);

        println!("\nturn of: {:?}", game.to_play());
        let mut buf = String::new();

        println!("enter row (number): ");
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let row = match buf.trim().parse::<usize>() {
            Ok(n) => Row::from(n),
            _ => continue,
        };

        println!("enter col (letter): ");
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let col = match buf.trim().chars().next() {
            Some(ch) => match ch {
                'A'..='Z' => Col::from(usize::from(ch as u8 - b'A')),
                'a'..='z' => Col::from(usize::from(ch as u8 - b'a')),
                _ => continue,
            },
            _ => continue,
        };

        println!("enter direction: [H/V]");
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        let dir = match buf.trim().chars().next() {
            Some('H') | Some('h') => Direction::Right,
            Some('V') | Some('v') => Direction::Down,
            _ => continue,
        };

        println!("enter word: ");

        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let word = match Word::new(buf.trim(), dir, (row, col)) {
            Some(word) => word,
            _ => continue,
        };

        let words = vec![word];
        let play = Play::place_words(words.into_iter());

        println!("{}", play);

        match game.make_play(play) {
            Ok(_) => println!("OK"),
            Err(e) => println!("{:?}", e),
        }
    }
}
