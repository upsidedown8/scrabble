# Scrabble AI

A Level Computer Science NEA project. This project
provides a Scrabble computer opponent, and a
real time online multiplayer service powered by 
[Warp](https://crates.io/crates/warp) and 
[Sycamore](https://crates.io/crates/sycamore).

# Project structure

The project is split into 4 crates (libraries):

## `client`
The client app. Single Page App which communicates with
the server using a REST API.

## `server`
The server: runs on a seperate machine and persists user/game
data, manages multiplayer games etc. Serves as a trustworthy
party to ensure that every move played is legal. Provides the
static build files for the client. A PostgreSQL database is used
(with [sqlx](https://crates.io/crates/sqlx)),
to store user and game data. A REST API is provided with
[Warp](https://crates.io/crates/warp) using the types from the `api`
crate, exchanging data with JSON.

## `scrabble`
Core library for Scrabble types, game modelling and AI. Self
contained module which is reused in `client` for local games
and in `server` for validating plays and managing game state.

## `api`
Rust types (structs, enums) which model the expected data
payload and response from the server for each API endpoint.
Using a single crate for the API ensures that the client and
server are always up to date (since the entire project is
developed with the same language). The API and interactions
with its types can therefore be type checked at compile time.
