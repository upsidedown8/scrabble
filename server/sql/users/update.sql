UPDATE
  tbl_user
SET
  username = ?,
  email = ?,
  hashed_pass = ?,
  is_private = ?,
  date_updated = ?
WHERE
  id_user = ?;