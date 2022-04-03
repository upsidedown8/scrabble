UPDATE tbl_player
SET is_winner = $1
WHERE tbl_player.id_player = $2;