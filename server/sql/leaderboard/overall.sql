SELECT AVG(play_summary.score) AS avg_score_per_play,
  AVG(play_summary.avg_word_length) AS avg_word_length,
  AVG(play_summary.word_count) AS avg_words_per_play,
  AVG(play_summary.tile_count) AS avg_tiles_per_play,
  MAX(play_summary.longest_word) AS longest_word,
  MAX(play_summary.score) AS best_word_score,
  SUM(play_summary.score) / COUNT(tbl_game.id_game) AS avg_score_per_game,
  SUM(play_summary.word_count) / COUNT(tbl_game.id_game) AS avg_words_per_game,
  SUM(play_summary.score) / SUM(play_summary.tile_count) AS avg_score_per_tile,
  COUNT(player_wins.id_player) / COUNT(tbl_game.id_game) * 100 AS win_percentage
FROM tbl_user
  JOIN tbl_player ON tbl_player.id_user = tbl_user.id_user
  JOIN tbl_game ON tbl_game.id_game = tbl_player.id_game
  JOIN tbl_player AS player_wins ON (
    player_wins.id_user = tbl_user.id_user
    AND player_wins.is_winner = TRUE
  ),
  (
    SELECT tbl_play.id_play AS id_play,
      COUNT(tbl_tile.pos) AS tile_count,
      COUNT(tbl_word.id_word) AS word_count,
      MAX(LENGTH(tbl_word.letters)) AS longest_word,
      AVG(LENGTH(tbl_word.letters)) AS avg_word_length,
      CASE
        WHEN COUNT(tbl_tile.pos) = 7 THEN 50
        ELSE 0
      END + SUM(tbl_word.score) AS score
    FROM tbl_play
      LEFT JOIN tbl_word ON tbl_word.id_play = tbl_play.id_play
      LEFT JOIN tbl_tile ON tbl_tile.id_play = tbl_play.id_play
    GROUP BY tbl_play.id_play
  ) AS play_summary
WHERE tbl_game.is_over = TRUE
  AND tbl_user.is_private = FALSE
GROUP BY tbl_user.id_user
ORDER BY win_percentage ASC
LIMIT $1 OFFSET $2;