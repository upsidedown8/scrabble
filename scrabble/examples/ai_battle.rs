use scrabble::{
    ai::Ai,
    game::{Game, GameStatus},
    util::fsm::FastFsm,
};
use std::{fs::File, io::BufReader};

fn main() {
    // Open the FSM file.
    let file = File::open("../server/data/fast_fsm.bin").unwrap();
    let rdr = BufReader::new(file);
    let fsm: FastFsm = bincode::deserialize_from(rdr).unwrap();

    // Create an easy AI.
    let ai = Ai::easy();
    let mut game = Game::new(4);

    // Let the AI play itself until the game is over.
    while let GameStatus::ToPlay(_) = game.status() {
        let play = ai.next_play(&fsm, &game);

        if let Err(e) = game.make_play(&play, &fsm) {
            eprintln!("err: {e}");
            break;
        }
    }

    // Display the final board state.
    println!("{}", game.board());
    println!("{:#?}", game.status());

    if let GameStatus::Over(game_over) = game.status() {
        for (winner, score) in game_over.winners() {
            println!("winner: {winner:?}; score: {score}");
        }
    }
}
