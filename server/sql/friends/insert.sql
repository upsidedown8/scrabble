INSERT INTO tbl_friend_request (from_id_user, to_id_user, date_sent)
SELECT $1 AS from_id_user,
    tbl_user.id_user AS to_id_user,
    $3 AS date_sent
FROM tbl_user
WHERE tbl_user.username = $2
LIMIT 1;