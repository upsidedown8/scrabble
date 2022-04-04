UPDATE tbl_game
SET is_over = TRUE,
    end_time = $2
WHERE id_game = $1;