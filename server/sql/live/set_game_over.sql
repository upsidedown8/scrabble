UPDATE tbl_game
SET is_over = TRUE
WHERE id_game = $1;