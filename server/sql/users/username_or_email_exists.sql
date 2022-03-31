SELECT COUNT(*)
FROM tbl_user
WHERE (
        username = $1
        OR email = $2
    )
    AND id_user != $3
LIMIT 1;