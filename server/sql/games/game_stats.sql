SELECT tbl_game.id_game,
    tbl_game.start_time,
    tbl_game.end_time,
    tbl_game.is_over,
    tbl_user.username,
    AVG(play_summary.score)::REAL AS avg_score_per_play,
    AVG(play_summary.avg_word_length)::REAL AS avg_word_length,
    AVG(play_summary.word_count)::REAL AS avg_words_per_play,
    AVG(play_summary.tile_count)::REAL AS avg_tiles_per_play,
    MAX(play_summary.longest_word)::INTEGER AS longest_word_length,
    MAX(play_summary.score)::INTEGER AS best_word_score,
    (
        SUM(play_summary.score) / SUM(play_summary.tile_count)
    )::REAL AS avg_score_per_tile,
    tbl_player.is_winner AS is_win
FROM tbl_user
    JOIN tbl_player ON tbl_player.id_user = $1
    JOIN tbl_game ON tbl_game.id_game = tbl_player.id_game,
    (
        SELECT tbl_play.id_play AS id_play,
            COUNT(tbl_tile.pos) AS tile_count,
            COUNT(tbl_word.id_word) AS word_count,
            MAX(LENGTH(tbl_word.letters)) AS longest_word,
            AVG(LENGTH(tbl_word.letters)) AS avg_word_length,
            CASE
                WHEN COUNT(tbl_tile.pos) = 7 THEN 50
                ELSE 0
            END + SUM(tbl_word.score) AS score,
            CASE
                WHEN tbl_player.is_winner THEN 1
                ELSE 0
            END AS win_count
        FROM tbl_play
            JOIN tbl_player ON tbl_play.id_player = tbl_player.id_player
            LEFT JOIN tbl_word ON tbl_word.id_play = tbl_play.id_play
            LEFT JOIN tbl_tile ON tbl_tile.id_play = tbl_play.id_play
        WHERE tbl_player.id_user = $1
            AND tbl_player.id_game = $2
        GROUP BY tbl_play.id_play,
            tbl_player.id_player
    ) AS play_summary
WHERE tbl_game.is_over = TRUE
GROUP BY tbl_user.id_user,
    tbl_player.id_player,
    tbl_game.id_game
LIMIT 1;