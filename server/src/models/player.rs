#[derive(Debug)]
pub struct PlayerModel {
    id_player: usize,
    id_game: usize,
    id_user: String,
    starting_rack: String,
    ai_difficulty: f32,
}
