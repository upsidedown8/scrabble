#[derive(Debug)]
pub struct PlayerModel {
    pub id_player: usize,
    pub id_game: usize,
    pub id_user: String,
    pub starting_rack: String,
    pub ai_difficulty: f32,
}
