CREATE TABLE tbl_user (
  id_user SERIAL,
  username TEXT NOT NULL,
  email TEXT NOT NULL,
  hashed_pass TEXT NOT NULL,
  role TEXT NOT NULL,
  is_private BOOLEAN NOT NULL,
  date_joined TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  date_updated TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id_user),
  CONSTRAINT valid_role CHECK(role IN ('user', 'admin'))
);
CREATE TABLE tbl_friend_request (
  from_id_user SERIAL,
  to_id_user SERIAL,
  date_sent TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (from_id_user, to_id_user),
  FOREIGN KEY (from_id_user) REFERENCES tbl_user(id_user) ON DELETE CASCADE,
  FOREIGN KEY (to_id_user) REFERENCES tbl_user(id_user) ON DELETE CASCADE
);
CREATE TABLE tbl_game(
  id_game SERIAL,
  start_time TIMESTAMP,
  end_time TIMESTAMP,
  is_over BOOLEAN DEFAULT FALSE NOT NULL,
  PRIMARY KEY (id_game)
);
CREATE TABLE tbl_player(
  id_player SERIAL,
  id_game SERIAL NOT NULL,
  is_winner BOOLEAN,
  PRIMARY KEY (id_player),
  FOREIGN KEY (id_game) REFERENCES tbl_game(id_game) ON DELETE CASCADE
);
CREATE TABLE tbl_ai_player(
  id_player SERIAL,
  ai_difficulty TEXT NOT NULL,
  PRIMARY KEY (id_player),
  FOREIGN KEY (id_player) REFERENCES tbl_player(id_player) ON DELETE CASCADE,
  CONSTRAINT valid_difficulty CHECK(ai_difficulty IN ('easy', 'medium', 'hard'))
);
CREATE TABLE tbl_human_player(
  id_player SERIAL,
  id_user SERIAL,
  PRIMARY KEY (id_player),
  FOREIGN KEY (id_user) REFERENCES tbl_user(id_user) ON DELETE CASCADE,
  FOREIGN KEY (id_player) REFERENCES tbl_player(id_player) ON DELETE CASCADE
);
CREATE TABLE tbl_play(
  id_play SERIAL,
  id_player SERIAL NOT NULL,
  PRIMARY KEY (id_play),
  FOREIGN KEY (id_player) REFERENCES tbl_player (id_player) ON DELETE CASCADE
);
CREATE TABLE tbl_tile(
  id_play SERIAL,
  pos INTEGER NOT NULL,
  letter CHAR NOT NULL,
  is_blank BOOLEAN NOT NULL,
  PRIMARY KEY (id_play, pos),
  FOREIGN KEY (id_play) REFERENCES tbl_play (id_play) ON DELETE CASCADE,
  CONSTRAINT valid_pos CHECK (
    pos >= 0
    AND pos < 225
  )
);
CREATE TABLE tbl_word(
  id_word SERIAL,
  id_play SERIAL NOT NULL,
  score INTEGER NOT NULL,
  letters VARCHAR(15) NOT NULL,
  PRIMARY KEY (id_word),
  FOREIGN KEY (id_play) REFERENCES tbl_play (id_play) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS tbl_password_reset(
  id_user SERIAL,
  secret_hex TEXT NOT NULL,
  valid_until TIMESTAMP NOT NULL,
  PRIMARY KEY (id_user),
  FOREIGN KEY (id_user) REFERENCES tbl_user (id_user) ON DELETE CASCADE
);