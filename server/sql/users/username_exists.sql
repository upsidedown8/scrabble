SELECT COUNT(*)
FROM tbl_user
WHERE username = $1
LIMIT 1;