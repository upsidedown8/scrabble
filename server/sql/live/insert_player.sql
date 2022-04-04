INSERT INTO tbl_player (id_game, id_user)
VALUES ($1, $2)
RETURNING id_player;