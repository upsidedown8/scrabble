CREATE TABLE IF NOT EXISTS tbl_friend(
  -- UUID of the user
  id_user TEXT NOT NULL,
  -- UUID of the friend
  id_friend TEXT NOT NULL,
  PRIMARY KEY (id_user, id_friend)
);