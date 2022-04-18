use scrabble::util::fsm::{FastFsm, FsmBuilder, SmallFsm};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // open the list of words
    let word_file = File::open("../server/data/words.txt").unwrap();
    let reader = BufReader::new(word_file);
    let mut builder = FsmBuilder::default();

    println!(":: Building finite state machine");

    // remove whitespace from each line then insert into the builder.
    for line in reader.lines().flatten() {
        builder.insert(line.trim());
    }

    // build a FastFsm and write to a file.
    println!(":: Writing file 'fast_fsm.bin'");
    let fast_fsm: FastFsm = builder.build();
    let bytes = bincode::serialize(&fast_fsm).unwrap();
    std::fs::write("fast_fsm.bin", &bytes).unwrap();

    // build a SmallFsm and write to a file.
    println!(":: Writing file 'small_fsm.bin'");
    let small_fsm = SmallFsm::from(fast_fsm);
    let bytes = bincode::serialize(&small_fsm).unwrap();
    std::fs::write("small_fsm.bin", &bytes).unwrap();
}
