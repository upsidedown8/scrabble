use scrabble::{
    ai::Ai,
    game::{Game, GameStatus},
    util::fsm::FastFsm,
};
use std::{fs::File, io::BufReader};

fn main() {
    let file = File::open("fast_fsm.bin").unwrap();
    let rdr = BufReader::new(file);
    let fsm: FastFsm = bincode::deserialize_from(rdr).unwrap();
    let ai = Ai::easy();
    let mut game = Game::with_players(4);

    while let GameStatus::ToPlay(_) = game.status() {
        let play = ai.next_play(&fsm, &game);

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
