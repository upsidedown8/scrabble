SELECT tbl_user.username AS username,
    tbl_friend_request.date_sent AS date_sent
FROM tbl_friend_request
    JOIN tbl_user ON tbl_friend_request.to_id_user = tbl_user.id_user
WHERE tbl_friend_request.from_id_user = $1
    AND tbl_friend_request.to_id_user NOT IN (
        SELECT tbl_friend_request.from_id_user AS existing_friend_id
        FROM tbl_friend_request
        WHERE tbl_friend_request.to_id_user = $1
    );