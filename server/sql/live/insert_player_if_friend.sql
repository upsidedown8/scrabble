-- $1: id_game
-- $2: id_user
-- $3: (owner) id_user
INSERT INTO tbl_player (id_game)
SELECT $1::INTEGER
WHERE $2::INTEGER = $3
    OR $2 IN (
        SELECT tbl_friend_request.to_id_user
        FROM tbl_friend_request
        WHERE tbl_friend_request.from_id_user = $3
    )
LIMIT 1
RETURNING id_player