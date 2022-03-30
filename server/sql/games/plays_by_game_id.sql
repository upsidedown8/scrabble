SELECT tbl_play.id_play AS id_play,
    tbl_play.id_player AS id_player,
    tbl_play.letters_removed AS letters_removed,
    tbl_play.letters_added AS letters_added
FROM tbl_play
    JOIN tbl_player ON tbl_play.id_play = tbl_player.id_player
    JOIN tbl_game ON tbl_game.id_game = tbl_player.id_game
WHERE tbl_game.id_game = $1;