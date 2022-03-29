UPDATE tbl_user
SET username = $1,
  email = $2,
  hashed_pass = $3,
  is_private = $4,
  date_updated = $5
WHERE id_user = $6