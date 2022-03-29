SELECT tbl_password_reset.*
FROM tbl_password_reset,
  tbl_user
WHERE tbl_user.username = $1
  AND tbl_user.id_user = tbl_password_reset.id_user