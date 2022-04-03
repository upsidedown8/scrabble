INSERT INTO tbl_player (id_game, ai_difficulty)
VALUES ($1, $2)
RETURNING tbl_player.id_player;