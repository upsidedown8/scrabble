UPDATE tbl_player
SET is_winner = TRUE
WHERE tbl_player.id_player = $1;