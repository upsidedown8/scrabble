INSERT INTO tbl_player (id_game, id_user)
SELECT $1::INTEGER,
    $2::INTEGER
WHERE $2 = $3
    OR $2 IN (
        SELECT tbl_friend_request.to_id_user
        FROM tbl_friend_request
        WHERE tbl_friend_request.from_id_user = $3
    )
LIMIT 1
RETURNING id_player;