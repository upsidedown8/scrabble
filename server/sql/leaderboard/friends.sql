-- Summary of stats for each player.
WITH game_summary AS (
  WITH play_summary AS (
    WITH tile_query AS (
      SELECT tbl_tile.id_play,
        COUNT(tbl_tile.pos) AS tile_count,
        CASE
          WHEN COUNT(tbl_tile.pos) = 7 THEN 50
          ELSE 0
        END AS bonus_score
      FROM tbl_tile
      GROUP BY tbl_tile.id_play
    ),
    word_query AS (
      SELECT tbl_word.id_play,
        COUNT(tbl_word.id_word) AS word_count,
        MAX(LENGTH(tbl_word.letters)) AS longest_word,
        AVG(LENGTH(tbl_word.letters)) AS avg_word_length,
        SUM(tbl_word.score) AS total_score
      FROM tbl_word
      GROUP BY tbl_word.id_play
    )
    SELECT tbl_play.id_player AS id_player,
      tile_query.tile_count AS tile_count,
      tile_query.bonus_score + word_query.total_score AS score,
      word_query.word_count AS word_count,
      word_query.longest_word AS longest_word,
      word_query.avg_word_length AS avg_word_length,
      (tile_query.bonus_score + word_query.total_score) / tile_query.tile_count AS avg_score_per_tile
    FROM tile_query,
      word_query,
      tbl_play
    WHERE tile_query.id_play = word_query.id_play
      AND tile_query.id_play = tbl_play.id_play
  )
  SELECT tbl_player.id_player,
    AVG(play_summary.score) AS avg_score,
    AVG(play_summary.avg_word_length) AS avg_word_length,
    AVG(play_summary.word_count) AS avg_word_count,
    AVG(play_summary.tile_count) AS avg_tile_count,
    MAX(play_summary.longest_word) AS longest_word,
    MAX(play_summary.score) AS best_word_score,
    SUM(play_summary.word_count) AS total_word_count,
    SUM(play_summary.score) AS total_score,
    AVG(play_summary.avg_score_per_tile) AS avg_score_per_tile,
    CASE
      WHEN tbl_player.is_winner THEN 1
      ELSE 0
    END AS win_count
  FROM play_summary,
    tbl_player
  WHERE play_summary.id_player = tbl_player.id_player
  GROUP BY tbl_player.id_player
) -- Summary of stats over each game.
SELECT tbl_user.username,
  AVG(game_summary.avg_score)::REAL AS avg_score_per_play,
  AVG(game_summary.avg_word_length)::REAL AS avg_word_length,
  AVG(game_summary.avg_word_count)::REAL AS avg_words_per_play,
  AVG(game_summary.avg_tile_count)::REAL AS avg_tiles_per_play,
  MAX(game_summary.longest_word)::INTEGER AS longest_word_length,
  MAX(game_summary.best_word_score)::INTEGER AS best_word_score,
  AVG(game_summary.total_score)::REAL AS avg_score_per_game,
  AVG(game_summary.total_word_count)::REAL AS avg_words_per_game,
  AVG(game_summary.avg_score_per_tile)::REAL AS avg_score_per_tile,
  (
    SUM(game_summary.win_count) / COUNT(tbl_game.id_game) * 100.0
  )::REAL AS win_percentage
FROM game_summary,
  tbl_friend_request
  JOIN tbl_friend_request AS friends ON (
    friends.from_id_user = tbl_friend_request.to_id_user
    AND friends.to_id_user = $1
  )
  JOIN tbl_user ON tbl_user.id_user = tbl_friend_request.to_id_user
  JOIN tbl_human_player ON tbl_human_player.id_user = tbl_user.id_user
  JOIN tbl_player ON tbl_player.id_player = tbl_human_player.id_player
  JOIN tbl_game ON tbl_game.id_game = tbl_player.id_game
WHERE tbl_game.is_over = TRUE
  AND tbl_friend_request.from_id_user = $1
  AND game_summary.id_player = tbl_player.id_player
GROUP BY tbl_user.id_user
ORDER BY win_percentage DESC
LIMIT 20;