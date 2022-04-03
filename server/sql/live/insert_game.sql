INSERT INTO tbl_game (start_time, end_time, is_over)
VALUES ($1, $2, $3)
RETURNING id_game;