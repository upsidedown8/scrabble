UPDATE tbl_player
SET is_winner = FALSE
WHERE tbl_player.id_player = $1;