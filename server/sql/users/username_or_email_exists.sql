SELECT COUNT(*)
FROM tbl_user
WHERE username = $1
    OR email = $2
LIMIT 1;