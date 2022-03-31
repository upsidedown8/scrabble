INSERT INTO tbl_password_reset
VALUES ($1, $2, $3) ON CONFLICT (id_user) DO
UPDATE
SET secret_hex = $2,
    valid_until = $3;