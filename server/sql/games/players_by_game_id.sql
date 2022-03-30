SELECT tbl_player.id_player AS id_player,
    tbl_user.username AS username
FROM tbl_player
    JOIN tbl_game ON tbl_player.id_game = tbl_game.id_game
    JOIN tbl_user ON tbl_player.id_user = tbl_user.id_user
WHERE tbl_game.id_game = $1;