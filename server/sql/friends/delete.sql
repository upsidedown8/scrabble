DELETE FROM tbl_friend_request
WHERE tbl_friend_request.from_id_user = $1
    AND tbl_friend_request.to_id_user IN (
        SELECT tbl_user.id_user
        FROM tbl_user
        WHERE tbl_user.username = $2
        LIMIT 1
    );