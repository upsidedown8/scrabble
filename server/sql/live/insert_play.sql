INSERT INTO tbl_play (id_player)
VALUES ($1)
RETURNING id_play;