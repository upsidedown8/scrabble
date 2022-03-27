CREATE TABLE IF NOT EXISTS tbl_user (
  id_user TEXT NOT NULL,
  username TEXT UNIQUE NOT NULL,
  email TEXT NOT NULL,
  hashed_pass TEXT NOT NULL,
  role TEXT NOT NULL,
  is_private BOOLEAN DEFAULT FALSE,
  PRIMARY KEY (id_user)
);
CREATE TABLE IF NOT EXISTS tbl_friend (
  first_id_user TEXT NOT NULL,
  second_id_user TEXT NOT NULL,
  PRIMARY KEY (first_id_user, second_id_user),
  FOREIGN KEY (first_id_user) REFERENCES tbl_user (id_user),
  FOREIGN KEY (second_id_user) REFERENCES tbl_user (id_user)
);
CREATE TABLE IF NOT EXISTS tbl_friend_request (
  from_id_user TEXT NOT NULL,
  to_id_user TEXT NOT NULl,
  PRIMARY KEY (from_id_user, to_id_user),
  FOREIGN KEY (from_id_user) REFERENCES tbl_user (id_user),
  FOREIGN KEY (to_id_user) REFERENCES tbl_user (id_user)
);
CREATE TABLE IF NOT EXISTS tbl_player(
  id_player INTEGER NOT NULL AUTO_INCREMENT,
  id_game INTEGER REFERENCES tbl_game(id_game) NOT NULL,
  id_user TEXT REFERENCES tbl_user(id_user),
  initial_rack TEXT NOT NULL,
  ai_difficulty TEXT,
  PRIMARY KEY (id_player)
);
CREATE TABLE IF NOT EXISTS tbl_game(
  id_game TEXT NOT NULL,
  start_time TEXT,
  end_time TEXT,
  is_over BOOLEAN DEFAULT FALSE,
  PRIMARY KEY (id_game)
);
CREATE TABLE IF NOT EXISTS tbl_tile(
  id_tile INTEGER NOT NULL AUTO_INCREMENT,
  id_play INTEGER NOT NULL,
  pos INTEGER NOT NULL,
  letter CHAR NOT NULL,
  is_blank BOOLEAN NOT NULL,
  PRIMARY KEY (id_tile),
  FOREIGN KEY (id_play) REFERENCES tbl_play (id_play)
);
CREATE TABLE IF NOT EXISTS tbl_word(
  id_word INTEGER NOT NULL AUTO_INCREMENT,
  id_play INTEGER NOT NULL,
  score INTEGER NOT NULL,
  letters TEXT NOT NULL,
  PRIMARY KEY (id_word),
  FOREIGN KEY (id_play) REFERENCES tbl_play (id_play)
);
CREATE TABLE IF NOT EXISTS tbl_play(
  id_play INTEGER NOT NULL AUTO_INCREMENT,
  id_player INTEGER NOT NULL,
  letters_added TEXT NOT NULL,
  PRIMARY KEY (id_play),
  FOREIGN KEY (id_player) REFERENCES tbl_player (id_player)
);
CREATE TABLE IF NOT EXISTS tbl_outcome(
  id_player INTEGER NOT NULL,
  final_score INTEGER NOT NULL,
  is_winner BOOLEAN NOT NULL,
  PRIMARY KEY (id_player),
  FOREIGN KEY (id_player) REFERENCES tbl_player (id_player)
);
CREATE TABLE IF NOT EXISTS tbl_password_reset(
  id_user TEXT NOT NULL,
  secret_hex TEXT NOT NULL,
  valid_until TIMESTAMP NOT NULL,
  PRIMARY KEY (id_user),
  FOREIGN KEY (id_user) REFERENCES tbl_user (id_user)
);