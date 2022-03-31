SELECT tbl_game.id_game AS id_game,
    tbl_game.start_time AS start_time,
    tbl_game.end_time AS end_time,
    tbl_game.is_over AS is_over
FROM tbl_player
    JOIN tbl_game ON tbl_player.id_game = tbl_game.id_game
WHERE tbl_player.id_user = $1
    AND tbl_game.id_game = $2
LIMIT 1;