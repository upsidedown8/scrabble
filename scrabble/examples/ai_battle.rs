use scrabble::{
    ai::{highest_scoring::HighestScoring, Ai},
    game::{Game, GameStatus},
    util::fsm::FastFsm,
};
use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("../server/data/fast_fsm.bin").unwrap();
    let rdr = BufReader::new(file);
    let fsm: FastFsm = bincode::deserialize_from(rdr).unwrap();
    let ai = HighestScoring::default();
    let mut game = Game::with_players(4);

    while let GameStatus::ToPlay(player_num) = game.status() {
        let board = game.board();
        let rack = game.player(*player_num).rack();
        let play = ai.select_play(&fsm, board, rack, ());

        if let Err(e) = game.make_play(&play, &fsm) {
            eprintln!("err: {e}");
            break;
        }
    }

    println!("{}", game.board());
    println!("{:#?}", game.status());

    if let GameStatus::Over(game_over) = game.status() {
        for (winner, score) in game_over.winners() {
            println!("winner: {winner:?}; score: {score}");
        }
    }
}
