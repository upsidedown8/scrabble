-- $1: id_game
INSERT INTO tbl_player (id_game)
VALUES ($1)
RETURNING id_player