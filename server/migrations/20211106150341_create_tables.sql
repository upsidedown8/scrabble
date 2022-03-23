CREATE TABLE IF NOT EXISTS tbl_user (
  id_user TEXT PRIMARY KEY NOT NULL,
  username TEXT UNIQUE NOT NULL,
  email TEXT NOT NULL,
  hashed_pass TEXT NOT NULL,
  role TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS tbl_player(
  id_player INTEGER PRIMARY KEY NOT NULL,
  id_game INTEGER REFERENCES tbl_game(id_game) NOT NULL,
  id_user TEXT REFERENCES tbl_user(id_user),
  starting_rack TEXT NOT NULL,
  ai_difficulty REAL
);
CREATE TABLE IF NOT EXISTS tbl_game(
  id_game INTEGER PRIMARY KEY NOT NULL,
  start TIMESTAMP,
  end TIMESTAMP,
  is_over BOOLEAN DEFAULT FALSE
);
CREATE TABLE IF NOT EXISTS tbl_tile(
  id_tile INTEGER PRIMARY KEY NOT NULL,
  id_play INTEGER REFERENCES tbl_play(id_play) NOT NULL,
  letter CHAR NOT NULL,
  is_blank BOOLEAN NOT NULL,
  pos INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS tbl_word(
  id_word INTEGER PRIMARY KEY NOT NULL,
  id_play INTEGER REFERENCES tbl_play(id_play) NOT NULL,
  score INTEGER NOT NULL,
  letters TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS tbl_play(
  id_play INTEGER PRIMARY KEY NOT NULL,
  id_player INTEGER REFERENCES tbl_player(id_player) NOT NULL,
  letters_added TEXT NOT NULL
);