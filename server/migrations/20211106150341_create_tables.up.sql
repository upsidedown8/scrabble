CREATE TABLE IF NOT EXISTS tbl_user (
    id_user INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(20) UNIQUE NOT NULL,
    password CHAR(32) NOT NULL,
    salt CHAR(32) NOT NULL
);

CREATE TABLE IF NOT EXISTS tbl_player(
    id_player INTEGER PRIMARY KEY NOT NULL,
    id_game INTEGER REFERENCES tbl_game(id_game) NOT NULL,
    id_user INTEGER REFERENCES tbl_user(id_user),
    starting_rack CHAR(7) NOT NULL,
    ai_difficulty FLOAT
);

CREATE TABLE IF NOT EXISTS tbl_game(
    id_game INTEGER PRIMARY KEY NOT NULL,
    start TIMESTAMP,
    end TIMESTAMP,
    is_over BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS tbl_tile(
    id_tile INTEGER PRIMARY KEY NOT NULL,
    id_move INTEGER REFERENCES tbl_move(id_move) NOT NULL,
    letter CHAR NOT NULL,
    pos_x INTEGER NOT NULL,
    pos_y INTEGER NOT NULL,
    is_blank BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS tbl_word(
    id_word INTEGER PRIMARY KEY NOT NULL,
    id_move INTEGER REFERENCES tbl_move(id_move),
    score INTEGER,
    content VARCHAR(15)
);

CREATE TABLE IF NOT EXISTS tbl_move(
    id_move INTEGER PRIMARY KEY NOT NULL,
    id_player INTEGER REFERENCES tbl_player(id_player) NOT NULL,
    letters_added VARCHAR(7) NOT NULL
);

