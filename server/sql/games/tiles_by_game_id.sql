SELECT tbl_tile.id_play AS id_play,
    tbl_tile.letter AS letter,
    tbl_tile.is_blank AS is_blank,
    tbl_tile.pos AS pos
FROM tbl_tile
    JOIN tbl_play ON tbl_tile.id_play = tbl_play.id_play
    JOIN tbl_player ON tbl_play.id_play = tbl_player.id_player
    JOIN tbl_game ON tbl_game.id_game = tbl_player.id_game
WHERE tbl_game.id_game = $1;