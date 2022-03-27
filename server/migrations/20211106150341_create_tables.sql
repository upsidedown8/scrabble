CREATE TABLE IF NOT EXISTS tbl_user (
  id_user TEXT NOT NULL,
  username TEXT UNIQUE NOT NULL,
  email TEXT NOT NULL,
  hashed_pass TEXT NOT NULL,
  role TEXT NOT NULL,
  is_private BOOLEAN DEFAULT FALSE,
  date_joined TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  date_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id_user),
  CONSTRAINT valid_role CHECK (role in ('User', 'Admin'))
);
CREATE TABLE IF NOT EXISTS tbl_friend (
  first_id_user TEXT NOT NULL,
  second_id_user TEXT NOT NULL,
  date_added TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (first_id_user, second_id_user),
  FOREIGN KEY (first_id_user) REFERENCES tbl_user (id_user),
  FOREIGN KEY (second_id_user) REFERENCES tbl_user (id_user)
);
CREATE TABLE IF NOT EXISTS tbl_friend_request (
  from_id_user TEXT NOT NULL,
  to_id_user TEXT NOT NULl,
  date_sent TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (from_id_user, to_id_user),
  FOREIGN KEY (from_id_user) REFERENCES tbl_user (id_user),
  FOREIGN KEY (to_id_user) REFERENCES tbl_user (id_user)
);
CREATE TABLE IF NOT EXISTS tbl_player(
  id_player INTEGER PRIMARY KEY AUTOINCREMENT,
  id_game INTEGER REFERENCES tbl_game(id_game) NOT NULL,
  id_user TEXT REFERENCES tbl_user(id_user),
  ai_difficulty TEXT,
  initial_rack TEXT NOT NULL,
  CONSTRAINT ai_xor_human CHECK (
    (
      id_user IS NULL
      AND ai_difficulty IN ('Easy', 'Medium', 'Hard')
    )
    OR (
      ai_difficulty IS NULL
      AND id_user NOT NULL
    )
  )
);
CREATE TABLE IF NOT EXISTS tbl_game(
  id_game TEXT NOT NULL,
  start_time TIMESTAMP,
  end_time TIMESTAMP,
  is_over BOOLEAN DEFAULT FALSE,
  PRIMARY KEY (id_game)
);
CREATE TABLE IF NOT EXISTS tbl_tile(
  id_play INTEGER NOT NULL,
  pos INTEGER NOT NULL,
  letter CHAR NOT NULL,
  is_blank BOOLEAN NOT NULL,
  PRIMARY KEY (id_play, pos),
  FOREIGN KEY (id_play) REFERENCES tbl_play (id_play),
  CONSTRAINT valid_pos CHECK (
    pos >= 0
    AND pos < 225
  )
);
CREATE TABLE IF NOT EXISTS tbl_word(
  id_word INTEGER PRIMARY KEY AUTOINCREMENT,
  id_play INTEGER NOT NULL,
  score INTEGER NOT NULL,
  letters TEXT NOT NULL,
  new_count INTEGER NOT NULL,
  FOREIGN KEY (id_play) REFERENCES tbl_play (id_play),
  CONSTRAINT valid_new_count CHECK(
    new_count > 0
    AND new_count <= LENGTH(letters)
    AND new_count <= 7
  ),
  CONSTRAINT valid_letter_count CHECK(LENGTH(letters) <= 15)
);
CREATE TABLE IF NOT EXISTS tbl_play(
  id_play INTEGER PRIMARY KEY AUTOINCREMENT,
  id_player INTEGER NOT NULL,
  letters_added TEXT NOT NULL,
  FOREIGN KEY (id_player) REFERENCES tbl_player (id_player),
  CONSTRAINT valid_added_count CHECK(LENGTH(letters_added) <= 7)
);
CREATE TABLE IF NOT EXISTS tbl_outcome(
  id_player INTEGER PRIMARY KEY NOT NULL,
  final_score INTEGER NOT NULL,
  is_winner BOOLEAN NOT NULL,
  FOREIGN KEY (id_player) REFERENCES tbl_player (id_player)
);
CREATE TABLE IF NOT EXISTS tbl_password_reset(
  id_user TEXT PRIMARY KEY NOT NULL,
  secret_hex TEXT NOT NULL,
  valid_until TIMESTAMP NOT NULL,
  FOREIGN KEY (id_user) REFERENCES tbl_user (id_user)
);