# Scrabble

My A level computer science NEA project. This project
provides a Scrabble computer opponent, as well as a
real time online multiplayer service powered by 
[Rocket](https://rocket.rs) and [Yew](https://yew.rs).

## Project structure

The project is split into 2 'workspaces' (seperate crates)
which can be independantly tested.

# `client`

The client end. Provides a web UI (with [Yew](https://yew.rs))
which communicates with the server using a REST API.

# `server`

The server: runs on a seperate machine and persists user/game
data, manages multiplayer games etc. The server also handles
the core implementation of the game of Scrabble such as
determining whether a move is legal, or game ai that determines
an optimal move in a position.

An sqlite3 database is used (via the
[sqlx](https://github.com/launchbadge/sqlx) library, to store
user and game data.

