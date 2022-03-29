/// A record in `tbl_word`.
#[derive(Debug)]
pub struct Word {
    /// Autoincrementing id for the record.
    pub id_word: i32,
    /// References the play in which the word was placed.
    pub id_play: i32,
    /// The score of the word.
    pub score: i32,
    /// The letters of the word.
    pub letters: String,
    /// The number of letters in the word that were newly placed.
    pub new_count: i32,
}
