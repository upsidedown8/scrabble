INSERT INTO tbl_user (username, email, hashed_pass, role, is_private)
VALUES ($1, $2, $3, $4, $5)
RETURNING id_user;