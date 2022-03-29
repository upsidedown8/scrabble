SELECT tbl_user.username AS username,
    GREATEST(
        tbl_friend_request.date_sent,
        other_friend_request.date_sent
    ) AS since
FROM tbl_friend_request
    JOIN tbl_user ON tbl_friend_request.from_id_user = tbl_user.id_user
    JOIN tbl_friend_request AS other_friend_request ON other_friend_request.from_id_user = $1
    AND other_friend_request.to_id_user = tbl_user.id_user
WHERE tbl_friend_request.to_id_user = $1